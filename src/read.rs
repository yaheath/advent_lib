use std::any::Any;
use std::fs::File;
use std::fmt;
use std::io::{self, BufRead, BufReader, IsTerminal, Lines, Stdin, StdinLock};
use std::iter;
use std::iter::Iterator;
use std::path::Path;
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;

const EOL: &[char] = &['\n', '\r'];

#[derive(Clone)]
pub enum ParseErr {
    Err(String),
    Warn(String),
    Skip,
}

impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErr::Err(msg) |
            ParseErr::Warn(msg) => write!(f, "{msg}"),
            ParseErr::Skip => write!(f, "[skipped]"),
        }
    }
}

lazy_static! {
    static ref STDIN: Stdin = io::stdin();
}
lazy_static! {
    static ref EXE_RE: Regex = Regex::new(r"^day(\d+)").unwrap();
}

fn stdinlock() -> StdinLock<'static> {
    STDIN.lock()
}

enum LineIters {
    File(Lines<BufReader<File>>),
    Stdin(Lines<StdinLock<'static>>),
}

pub struct LineIter(LineIters);

impl Iterator for LineIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            LineIters::File(i) => i.next().map(|x| x.unwrap()),
            LineIters::Stdin(i) => i.next().map(|x| x.unwrap()),
        }
    }
}

pub fn input_lines() -> impl Iterator<Item=String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1]).unwrap();
        LineIter (
            LineIters::File(
                BufReader::new(file).lines()
            ),
        )
    }
    else if !STDIN.is_terminal() {
        let lock = stdinlock();
        LineIter (
            LineIters::Stdin(lock.lines()),
        )
    }
    else {
        let exe_name = Path::new(&args[0]).file_name().unwrap();
        let inp_name =
            if let Some(m) = EXE_RE.captures(exe_name.to_str().unwrap()) {
                format!("day{}.input", m.get(1).unwrap().as_str())
            }
            else {
                format!("{}.input", Path::new(&args[0]).file_stem().unwrap().to_str().unwrap())
            };
        let file = File::open(inp_name).unwrap();
        LineIter (
            LineIters::File(
                BufReader::new(file).lines()
            ),
        )
    }
}

impl ParseErr {
    fn show_err(&self, line: &str) {
        match self {
            ParseErr::Err(e) => {
                panic!("Invalid line: {}\nError: {e}", line.trim());
            },
            ParseErr::Warn(e) => {
                eprintln!("Invalid line: {}\nError: {e}", line.trim());
            },
            ParseErr::Skip => {},
        }
    }
}

fn parse_line<T>(line: &str) -> Option<T>
where T: FromStr, <T as FromStr>::Err: Any {
    match line.trim_end_matches('\n').parse::<T>() {
        Ok(val) => Some(val),
        Err(e) => {
            let e_any = &e as &dyn Any;
            if let Some(pe) = e_any.downcast_ref::<ParseErr>() {
                pe.show_err(line);
            }
            else {
                eprintln!("Invalid line: {}", line.trim());
            }
            None
        },
    }
}

pub fn input_from_iter<T, I: Iterator<Item=String>>(line_iter: I) -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Any {
    line_iter
        .flat_map(|l| parse_line(l.as_str()))
        .collect()
}

pub fn input_as_string() -> String {
    input_lines()
        .chain(iter::once("".into()))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn test_input<T: FromStr>(data: &str) -> Vec<T>
where <T as FromStr>::Err: Any {
    input_from_iter(data.lines().map(|l| l.into()))
}

pub fn read_input<T: FromStr>() -> Vec<T>
where <T as FromStr>::Err: Any {
    input_from_iter(input_lines())
}

pub fn grouped_input_from_iter<T: FromStr, I: Iterator<Item=String>>(line_iter: I) -> Vec<Vec<T>>
where <T as FromStr>::Err: Any {
    let mut data: Vec<Vec<T>> = Vec::new();
    let mut row: Vec<T> = Vec::new();
    for line in line_iter {
        let val = line.trim_end_matches(EOL);
        if val.is_empty() {
            data.push(row);
            row = Vec::new();
        }
        else if let Some(v) = parse_line(val) {
            row.push(v);
        }
    };
    if !row.is_empty() {
        data.push(row);
    }
    data
}

pub fn read_grouped_input<T: FromStr>() -> Vec<Vec<T>>
where <T as FromStr>::Err: Any {
    grouped_input_from_iter(input_lines())
}

pub fn grouped_test_input<T: FromStr>(data: &str) -> Vec<Vec<T>>
where <T as FromStr>::Err: Any {
    grouped_input_from_iter(data.lines().map(|l| l.into()))
}

pub fn sectioned_input_from_iter<T1: FromStr, T2: FromStr, I: Iterator<Item=String>>(mut line_iter: I) -> (Vec<T1>,Vec<T2>)
where <T1 as FromStr>::Err: Any, <T2 as FromStr>::Err: Any {
    let mut data1: Vec<T1> = Vec::new();
    let mut data2: Vec<T2> = Vec::new();

    for l in line_iter.by_ref() {
        let l = l.trim_end_matches(EOL);
        if l.is_empty() { break; }
        if let Some(v) = parse_line(l) {
            data1.push(v);
        }
    }
    for l in line_iter {
        let l = l.trim_end_matches(EOL);
        if let Some(v) = parse_line(l) {
            data2.push(v);
        }
    }
    (data1, data2)
}

pub fn read_sectioned_input<T1: FromStr, T2: FromStr>() -> (Vec<T1>, Vec<T2>)
where <T1 as FromStr>::Err: Any, <T2 as FromStr>::Err: Any {
    sectioned_input_from_iter(input_lines())
}

pub fn sectioned_test_input<T1: FromStr, T2: FromStr>(data: &str) -> (Vec<T1>, Vec<T2>)
where <T1 as FromStr>::Err: Any, <T2 as FromStr>::Err: Any {
    sectioned_input_from_iter(data.lines().map(|l| l.into()))
}
