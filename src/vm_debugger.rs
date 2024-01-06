use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::io;
use std::iter;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use itertools::Itertools;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use crate::vm_shell::{VMShell, CPU, RunResult};
use crate::vm_display::{InstructionDisplay, Formatter, Token};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const OPCODE_COLOR:Color = Color::Red;
const REG_COLOR:Color = Color::Blue;
const INT_COLOR:Color = Color::Cyan;
const ADDR_COLOR:Color = Color::Green;
const BP_COLOR:Color = Color::Blue;

lazy_static! {
    static ref STYLES: HashMap<char, Style> = HashMap::from_iter([
        ('o', Style::default().fg(OPCODE_COLOR)),
        ('r', Style::default().fg(REG_COLOR)),
        ('i', Style::default().fg(INT_COLOR)),
        ('a', Style::default().fg(ADDR_COLOR)),
        ('b', Style::default().fg(BP_COLOR)),
    ]);
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum RunState {
    Pause,
    Walk,
    Run,
    Halt,
    Error,
}

enum Focus {
    Normal,
    BreakpointInput,
}

pub struct Debugger<'a, RegKey, RegVal: TryFrom<usize>, Instr> {
    shell: &'a mut VMShell<RegKey, RegVal, Instr>,
    cpu: &'a dyn CPU<RegKey, RegVal, Instr>,
    run_state: RunState,
    walk_delay: Duration,
    breakpoint: bool,
    pc_breakpoints: HashSet<usize>,
    hex_mode: bool,
    focus: Focus,
    pc_input: Input,
}

impl<
    'a,
    RegKey: Clone + Hash + Eq + Ord + PartialOrd + Into<String>,
    RegVal: Copy + fmt::Display + fmt::LowerHex + fmt::UpperHex + TryFrom<usize> + Copy + Clone,
    Instr: Clone + InstructionDisplay<RegVal>,
> Debugger<'a, RegKey, RegVal, Instr> {

    pub fn run(shell: &'a mut VMShell<RegKey, RegVal, Instr>, cpu: &'a dyn CPU<RegKey, RegVal, Instr>) -> Result<()> {
        let terminal = Self::init_terminal()?;
        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic| {
            Self::reset_terminal().unwrap();
            original_hook(panic);
        }));

        let mut inst = Self {
            shell,
            cpu,
            run_state: RunState::Pause,
            walk_delay: Duration::from_millis(10),
            breakpoint: false,
            hex_mode: false,
            pc_breakpoints: HashSet::new(),
            focus: Focus::Normal,
            pc_input: Input::default(),
        };
        inst.run_impl(terminal)?;

        Ok(())
    }

    fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(terminal)
    }

    fn reset_terminal() -> Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn is_halted(&self) -> bool {
        matches!(self.run_state, RunState::Halt | RunState::Error)
    }

    fn run_impl<B: Backend>(&mut self, mut terminal: Terminal<B>) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            if self.run_state != RunState::Run {
                terminal.draw(|f| self.ui(f))?;
            }
            self.breakpoint = false;

            let timeout = if self.run_state == RunState::Walk {
                self.walk_delay
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0))
            } else { Duration::from_secs(0) };

            let mut do_step = false;

            if matches!(self.focus, Focus::BreakpointInput) {
                let e = event::read()?;
                if let Event::Key(key) = e {
                    match key.code {
                        KeyCode::Enter => {
                            if let Ok(n) = self.pc_input.value().parse::<usize>() {
                                if n < self.shell.vm.program.len() {
                                    if self.pc_breakpoints.contains(&n) {
                                        self.pc_breakpoints.remove(&n);
                                    }
                                    else {
                                        self.pc_breakpoints.insert(n);
                                    }
                                }
                            }
                            self.pc_input.reset();
                            self.focus = Focus::Normal;
                        },
                        KeyCode::Esc => {
                            self.pc_input.reset();
                            self.focus = Focus::Normal;
                        },
                        _ => {
                            self.pc_input.handle_event(&e);
                        },
                    }
                }
            }
            else if self.run_state != RunState::Walk && self.run_state != RunState::Run || event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('s') => { self.cmd_pause(); do_step = true; },
                        KeyCode::Char('r') => {
                            self.cmd_run();
                            terminal.draw(|f| self.ui(f))?;
                        },
                        KeyCode::Char('w') => self.cmd_walk(),
                        KeyCode::Char('p') => self.cmd_pause(),
                        KeyCode::Char('h') => {
                            self.hex_mode = !self.hex_mode;
                            if self.run_state == RunState::Run {
                                terminal.draw(|f| self.ui(f))?;
                            }
                        },
                        KeyCode::Char('b') => {
                            self.cmd_pause();
                            self.focus = Focus::BreakpointInput;
                        }
                        KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
            }

            if match self.run_state {
                RunState::Run => true,
                RunState::Pause => do_step,
                RunState::Walk => {
                    if last_tick.elapsed() >= self.walk_delay {
                        last_tick = Instant::now();
                        true
                    } else {
                        false
                    }
                },
                _ => false,
            } {
                let rr = self.shell.step(self.cpu);
                match rr {
                    RunResult::Break => {
                        self.run_state = RunState::Pause;
                        self.breakpoint = true;
                    },
                    RunResult::Halt => {
                        self.run_state = RunState::Halt;
                    },
                    RunResult::Err => {
                        self.run_state = RunState::Error;
                    },
                    _ => {
                        if self.pc_breakpoints.contains(&self.shell.vm.pc) {
                            self.run_state = RunState::Pause;
                            self.breakpoint = true;
                        }
                    },
                }
            }
        }
        let _ = Self::reset_terminal();
        Ok(())
    }

    fn cmd_run(&mut self) {
        if !self.is_halted() {
            self.run_state = RunState::Run;
        }
    }
    fn cmd_walk(&mut self) {
        if !self.is_halted() {
            self.run_state = RunState::Walk;
        }
    }
    fn cmd_pause(&mut self) {
        if !self.is_halted() {
            self.run_state = RunState::Pause;
        }
    }

    fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        let (statuspanel, sp_height) = self.render_statuspanel();
        let root = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref())
            .split(f.size());
        let rightside = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(sp_height),
            ].as_ref())
            .split(root[1]);
        self.render_code(f, root[0]);
        self.render_registers(f, rightside[0]);
        f.render_widget(statuspanel, rightside[1]);
        if matches!(self.focus, Focus::BreakpointInput) {
            let rect = Rect::new(
                rightside[1].x + 20,
                rightside[1].y + 3,
                rightside[1].width - 23,
                3);
            self.render_pc_input(f, rect);
        }
    }

    fn render_statuspanel(&self) -> (List, u16) {
        let items:Vec<ListItem> = vec![
            ListItem::new("[p] Pause           [q] Quit"),
            ListItem::new("[s] Step"),
            ListItem::new("[w] Walk            [b] Breakpoint"),
            ListItem::new("[r] Run"),
            ListItem::new("[h] Toggle decimal/hex"),
        ];
        let title = match (self.breakpoint, self.run_state) {
            (true, _) => "BREAKPOINT",
            (false, RunState::Pause) => "PAUSED",
            (false, RunState::Walk) => "WALKING",
            (false, RunState::Run) => "RUNNING",
            (false, RunState::Halt) => "HALTED",
            (false, RunState::Error) => "ERROR",
        };
        let height = items.len() as u16 + 2;

        (
            List::new(items)
                .block(Block::default().title(title).borders(Borders::ALL)),
            height,
        )
    }

    fn render_pc_input<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) {
        let scroll = self.pc_input.visual_scroll((rect.width - 2) as usize);
        let input = Paragraph::new(self.pc_input.value())
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("Enter address"));
        f.render_widget(input, rect);
        f.set_cursor(
            rect.x
            + ((self.pc_input.visual_cursor()).max(scroll) - scroll) as u16
            + 1,
            rect.y + 1,
        );
    }

    fn fmtint(&self, val: &RegVal) -> String {
        if self.hex_mode {
            format!("{val:#x}")
        } else {
            format!("{val}")
        }
    }

    fn token_list(&self, iter: &mut dyn Iterator<Item=Vec<Token<RegVal>>>) -> Vec<Line> {
        let rows: Vec<Vec<Span>> = iter
            .map(|row| row.iter().map(
                |t| match t {
                    Token::Opcode(s) => Span::styled(s.clone(), STYLES[&'o']),
                    Token::Register(s) => Span::styled(s.clone(), STYLES[&'r']),
                    Token::Integer(i) => Span::styled(self.fmtint(i), STYLES[&'i']),
                    Token::Address(i) => Span::styled(self.fmtint(i), STYLES[&'a']),
                })
                .collect()
            )
            .collect();

        let maxcols = rows.iter().map(|r| r.len()).max().unwrap();
        let mut colwidths: Vec<usize> = vec![0; maxcols];
        rows.iter().for_each(|row| {
            row.iter().enumerate().for_each(|(idx, t)| colwidths[idx] = colwidths[idx].max(t.width()));
        });

        rows.iter()
            .map(|row| Line::from(
                    row.iter()
                    .enumerate()
                    .flat_map(|(idx, t)| {
                        let w = t.width();
                        let pad = Span::raw(" ".repeat(colwidths[idx] - w));
                        let mut spans = if idx == 0 {
                            vec![pad, t.clone()]
                        } else {
                            vec![t.clone(), pad]
                        };
                        if idx < row.len() - 1 {
                            spans.push(Span::raw(" "));
                        }
                        spans
                    })
                    .collect::<Vec<_>>()
                )
            )
            .collect()
    }

    fn render_code<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) {
        let mut itr = self.shell.vm.program
            .iter()
            .enumerate()
            .map(|(idx, instr)| {
                let mut fmt = Formatter::new();
                instr.fmt(&mut fmt);
                iter::once(&Token::Address(idx.try_into().ok().unwrap()))
                    .chain(fmt.get_tokens())
                    .cloned()
                    .collect()
            });
        let rows: Vec<ListItem> = self.token_list(&mut itr)
            .into_iter()
            .enumerate()
            .map(|(idx, mut line)| {
                let bp: String = if self.pc_breakpoints.contains(&idx) {
                    "\u{25c8} ".into()
                } else {
                    "  ".into()
                };
                line.spans.insert(0, Span::styled(bp, STYLES[&'b']));
                ListItem::new(line)
            })
            .collect();

        let list = List::new(rows)
            .block(Block::default().borders(Borders::ALL).title("Program"))
            .highlight_style(
                Style::default().add_modifier(Modifier::REVERSED)
            );
        let mut liststate = ListState::default();
        if self.shell.vm.pc < self.shell.vm.program.len() {
            liststate.select(Some(self.shell.vm.pc));
        }
        f.render_stateful_widget(list, rect, &mut liststate);
    }

    fn render_registers<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) {
        let mut itr = self.shell.vm.registers
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .map(|(k, v)| vec![
                Token::Register(k.clone().into()),
                Token::Integer(*v),
            ]);
        let rows: Vec<ListItem> = self.token_list(&mut itr)
            .into_iter()
            .map(ListItem::new)
            .collect();

        let list = List::new(rows)
            .block(Block::default().borders(Borders::ALL).title("Registers"));

        f.render_widget(list, rect);
    }
}
