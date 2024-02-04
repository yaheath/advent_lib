use std::collections::HashMap;
use std::hash::Hash;
use std::vec::Vec;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum InstructionResult {
    Ok,
    Halt,
    Break,
    Err,
    JumpFwd(usize),
    JumpBck(usize),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum RunResult {
    Ok,
    Halt,
    Break,
    Err,
}

pub trait CPU<RegKey, RegVal: TryFrom<usize>, Instr> {
    fn execute_instruction(
        &self,
        vm: &mut VM<RegKey, RegVal, Instr>,
        i: &Instr,
    ) -> InstructionResult;
}

pub struct VM<RegKey, RegVal: TryFrom<usize>, Instr> {
    pub program: Vec<Instr>,
    pub registers: HashMap<RegKey, RegVal>,
    pub register_default: RegVal,
    pub pc: usize,
}

impl<RegKey: Hash + Eq, RegVal: Copy + TryFrom<usize>, Instr> VM<RegKey, RegVal, Instr> {
    fn new(program: Vec<Instr>, register_default: RegVal) -> Self {
        Self {
            program,
            registers: HashMap::new(),
            register_default,
            pc: 0,
        }
    }
    fn is_halted(&self) -> bool {
        self.pc >= self.program.len()
    }
    fn reset(&mut self) {
        self.registers.clear();
        self.pc = 0;
    }
    pub fn get_reg(&self, r: RegKey) -> RegVal {
        self.registers
            .get(&r)
            .copied()
            .unwrap_or(self.register_default)
    }
    pub fn set_reg(&mut self, r: RegKey, v: RegVal) {
        self.registers.entry(r).and_modify(|p| *p = v).or_insert(v);
    }
}

pub struct VMShell<RegKey, RegVal: TryFrom<usize>, Instr> {
    pub vm: VM<RegKey, RegVal, Instr>,
}

impl<RegKey: Hash + Eq + Ord + PartialOrd, RegVal: Copy + TryFrom<usize>, Instr: Clone>
    VMShell<RegKey, RegVal, Instr>
{
    pub fn new(program: Vec<Instr>, register_default: RegVal) -> Self {
        let vm = VM::new(program, register_default);
        Self { vm }
    }

    pub fn reset(&mut self) {
        self.vm.reset();
    }

    pub fn step(&mut self, cpu: &dyn CPU<RegKey, RegVal, Instr>) -> RunResult {
        if self.vm.is_halted() {
            return RunResult::Halt;
        }
        let inst = self.vm.program[self.vm.pc].clone();
        match cpu.execute_instruction(&mut self.vm, &inst) {
            InstructionResult::Ok => {
                self.vm.pc += 1;
            }
            InstructionResult::Halt => {
                self.vm.pc = usize::MAX;
                return RunResult::Halt;
            }
            InstructionResult::Break => {
                self.vm.pc += 1;
                return RunResult::Break;
            }
            InstructionResult::Err => {
                return RunResult::Err;
            }
            InstructionResult::JumpFwd(n) => {
                self.vm.pc = self.vm.pc.saturating_add(n);
            }
            InstructionResult::JumpBck(n) => {
                self.vm.pc = self.vm.pc.overflowing_sub(n).0;
            }
        }
        if self.vm.pc > self.vm.program.len() {
            self.vm.pc = usize::MAX;
            RunResult::Halt
        } else {
            RunResult::Ok
        }
    }

    pub fn run(&mut self, cpu: &dyn CPU<RegKey, RegVal, Instr>) -> RunResult {
        loop {
            let r = self.step(cpu);
            match r {
                RunResult::Ok => {}
                _ => {
                    return r;
                }
            }
        }
    }
}
