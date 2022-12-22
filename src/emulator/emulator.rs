use std::time::Duration;
use devices::DeviceHost;

use wasm_bindgen::prelude::*;
use crate::emulator::ast::Parser;

use super::{*, lexer, ast::{self, Inst, Program, Operand}};

#[derive(Debug)]
pub enum EmulatorErrorKind {
    StackOverflow,
    StackUnderflow,
}

impl <'a> std::fmt::Display for EmulatorErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmulatorErrorKind::StackOverflow => write!(f, "Stack overflow"),
            EmulatorErrorKind::StackUnderflow => write!(f, "Stack underflow"),
        }
    }
}

#[derive(Debug)]
pub struct EmulatorError(Option<EmulatorErrorKind>);

impl EmulatorError {
    fn new() -> Self {
        EmulatorError(None)
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct EmulatorState {
    regs: Vec<u64>,
    heap: Vec<u64>,
    stack: Stack,
    pc: usize,
    program: Program,
    devices: DeviceHost,
    error: EmulatorError,
}

#[derive(Debug)]
pub struct Stack {
    data: Vec<u64>,
    sp: i64,
    size: usize,
}

impl Stack {
    fn new(size: usize) -> Self {
        let mut data = Vec::new();
        data.resize(size, 0);
        Stack { data, sp: 0, size }
    }
    
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq)]
pub enum StepResult {
    Continue, HLT, Input,
}

pub const PC: u64 = u64::MAX;
pub const SP: u64 = u64::MAX - 1;

// you cant bindgen impls i dont think
#[wasm_bindgen]
#[allow(dead_code)]
impl EmulatorState {

    fn new(program: Program, devices: DeviceHost) -> Self {
        let regs = vec![0; program.headers.minreg as usize];
        let heap = vec![0; (program.headers.minheap + program.headers.minstack) as usize];
        EmulatorState { regs, heap, stack: Stack::new(program.headers.minstack as usize), pc: 0, program, devices, error: EmulatorError::new() }
    }
    
    fn get(&self, operand: &Operand) -> u64 {
        match operand {
            Operand::Imm(v) => *v,
            Operand::Reg(v) => {
                match *v {
                    PC => self.pc as u64,
                    SP => self.stack.data.len() as u64,
                    _  => self.regs[*v as usize]
                }
            },
            _ => panic!("Unsupported operand {:?}", operand)
        }
    }
    fn set(&mut self, operand: &Operand, value: u64) {
        match operand {
            Operand::Imm(_) => {}, // do nothing assume it is r0
            Operand::Reg(v) => {
                match *v {
                    PC => self.pc = value as usize,
                    SP => self.stack.sp = value as i64,
                    _  => self.regs[*v as usize] = value
                }
            },
            _ => panic!("Unsupported target operand {:?}", operand)
        }
    }
    
    fn getm(&self, operand: &Operand) -> u64 {
        let index = self.get(operand) as usize;
        self.heap[index]
    }
    fn setm(&mut self, operand: &Operand, value: u64) {
        let index = self.get(operand) as usize;
        self.heap[index] = value;
    }

    fn push(&mut self, data: u64) {
        if self.stack.sp >= self.stack.size as i64 || self.stack.sp < 0 {
            
        }
        self.stack.data[self.stack.sp as usize] = data;
        self.stack.sp += 1;
    }
    fn pop(&mut self) -> u64 {
        if self.stack.sp > self.stack.size as i64 || self.stack.sp <= 0 {
            todo!();
        }
        self.stack.sp -= 1;
        self.stack.data[self.stack.sp as usize]
    }
    
    pub fn run(&mut self) -> StepResult {
        loop {
            let result = self.step();
            if result != StepResult::Continue {
                self.devices.show();
                return result;
            }
        }
    }
    fn run_for(&mut self, max_time: Duration) -> StepResult {
        return self.run_for_ms(max_time.as_secs_f64() * 1000.0);
    }
    pub fn show(&mut self) {
        clear_text();
        self.devices.show();
        jsprintln!("Regs: {:?}", self.regs);
    }
    
    pub fn run_for_ms(&mut self, max_time_ms: f64) -> StepResult {
        const BURST_LENGTH: u32 = 1024;
        let start = now();
        let end = start + max_time_ms;
        while now() < end {
            for _ in 0..BURST_LENGTH {
                let result = self.step();
                if result != StepResult::Continue {
                    self.show();
                    return result;
                }
            }
        }
        self.show();
        StepResult::Continue
    }
    // or maybe we just run on a sepperate thread 🤔 good idea
    // lets implement that
    
    // now how did multitrheading work on the web again lol
    // is there some cargo library for that or should we just do some Worker schenenigans
    

    pub fn step(&mut self) -> StepResult {
        // safety: pc is bounds checked here 
        if self.pc >= self.program.instructions.len() {
            return StepResult::HLT;
        }
        use Inst::*;
        // safety: pc has to bounds checked before hand
        match unsafe{fuck_borrow_checker(self.program.instructions.get_unchecked(self.pc))} {
            HLT => return StepResult::HLT,
            ADD(op1, op2, op3) => {
                self.set(op1, self.get(op2)+self.get(op3));
            },
            RSH(a, b) => self.set(a, self.get(b) >> 1),
            LOD(a, b) => self.set(a, self.getm(b)),
            STR(a, b) => self.setm(a, self.get(b)),
            BGE(a, b, c) => {
                if self.get(b) >= self.get(c) {
                    self.pc = self.get(a) as usize - 1;
                }
            },
            NOR(a, b, c) => self.set(a, u64::MAX - (self.get(b) | self.get(c))),
            MOV(a, b) => self.set(a, self.get(b)),
            INC(a, b) => self.set(a, self.get(b)+1),
            DEC(a, b) => self.set(a, self.get(b)-1),
            OUT(a, b) => self.devices.out(self.get(a), self.get(b)),
            IN(_a,_b) => todo!(),
            JMP(a) => self.pc = self.get(a) as usize - 1,
            SUB(a, b, c) => self.set(a, self.get(b) - self.get(c)),
            NOP => {},
            LSH(a, b) => self.set(a, self.get(b) << 1),
            NEG(a, b) => self.set(a, (-(self.get(b) as i64)) as u64),
            AND(a, b, c) => self.set(a, self.get(b) & self.get(c)),
            OR(a, b, c) => self.set(a, self.get(b) | self.get(c)),
            NOT(a, b) => self.set(a, !self.get(b)),
            NAND(a, b, c) => self.set(a, !(self.get(b) & self.get(c))),
            CPY(a, b) => self.setm(a, self.getm(b)),
            MLT(a, b, c) => self.set(a, self.get(b)*self.get(c)),
            DIV(a, b, c) => self.set(a, self.get(b)/self.get(c)),
            MOD(a, b, c) => self.set(a, self.get(b)%self.get(c)),
            ABS(a, b) => self.set(a, (self.get(b) as i64).abs() as u64),
            LLOD(a, b, c) => self.set(a, self.getm(&Operand::Imm(self.get(b) + self.get(c)))),
            LSTR(a, b, c) => self.setm(&Operand::Imm(self.get(a)+self.get(b)), self.get(c)),
            SDIV(a, b, c) => self.set(a, ((self.get(b) as i64)/(self.get(c) as i64)) as u64),
            SETE(a, b, c) => self.set(a, 
                if self.get(b) == self.get(c) {
                    u64::MAX
                } else {
                    0
                }
            ),
            SETG(a, b, c) => self.set(a,
                if self.get(b) > self.get(c) {
                    u64::MAX
                } else {
                    0
                }
            ),
            SETGE(a, b, c) => self.set(a,
                if self.get(b) >= self.get(c) {
                    u64::MAX
                } else {
                    0
                }
            ),
            SETL(a, b, c) => self.set(a,
                if self.get(b) < self.get(c) {
                    u64::MAX
                } else {
                    0
                }
            ),
            SETLE(a, b, c) => self.set(a,
                if self.get(b) <= self.get(c) {
                    u64::MAX
                } else {
                    0
                }
            ),
            XOR(a, b, c) => self.set(a, self.get(b)^self.get(c)),
            XNOR(a, b, c) => self.set(a, !(self.get(b)^self.get(c))),
            BNE(a, b, c) => {
                if self.get(b) != self.get(c) {
                    self.pc = self.get(a) as usize;
                }
            },
            BRE(a, b, c) => {
                if self.get(b) == self.get(c) {
                    self.pc = self.get(a) as usize;
                }
            },
            PSH(a) => {
                self.push(self.get(a));
            },
            POP(a) => {
                let b = self.pop();
                self.set(a, b);
            }
            _ => jsprintln!("Unimplimented instruction."),
        }
        self.pc += 1;

        StepResult::Continue
    }
}

#[inline]
unsafe fn fuck_borrow_checker<T>(a: *const T) -> &'static T { // no 
    &*a
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn emulate(src: &str) -> Option<EmulatorState> { // wifi died
    clear_text();
    let toks = lexer::lex(src);

    let Parser {ast: program, err, ..} = ast::gen_ast(toks);
    jsprintln!("{:#?}", program);
    jsprintln!("{}", err.to_string(src));
    if err.has_error() {
        return None;
    }

    let host = DeviceHost::new();
    let emu = EmulatorState::new(program, host);

    return Some(emu);
    // i can add more insts while you guys do that
    // also please do the too late label thing
    // we need to get some web workers out 👷‍♂️
    // let result = emu.run_for(Duration::from_millis(1000));
    // if result == StepResult::Continue {
    //     jsprintln!("Program took too long");
    // } else {
    //     jsprintln!("Program Halted");
    // }
    // emu.devices.show();
    // return emu;
    // for _ in 0..100 {
    //     jsprintln!("{:?}: {:?}", emu.pc, emu.regs);
    //     if emu.step() != StepResult::Continue {
    //         break;
    //     }
    // }
}