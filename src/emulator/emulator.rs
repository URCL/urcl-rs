use std::time::Duration;
use devices::DeviceHost;

use wasm_bindgen::prelude::*;
use super::{*, lexer, ast::{self, Inst, Program, Operand}};
#[derive(Debug)]
struct EmulatorState {
    regs: Vec<u64>,
    heap: Vec<u64>,
    pc: usize,
    program: Program,
    devices: DeviceHost,
}

#[derive(Debug, PartialEq, Eq)]
enum StepResult {
    Continue, HLT, Input,
}

impl EmulatorState {
    pub fn new(program: Program, devices: DeviceHost) -> Self {
        let regs = vec![0; program.headers.minreg as usize];
        let heap = vec![0; (program.headers.minheap + program.headers.minstack) as usize];
        EmulatorState { regs, heap, pc: 0, program, devices }
    }
    
    fn get(&self, operand: &Operand) -> u64 {
        match operand {
            Operand::Imm(v) => *v,
            Operand::Reg(v) => self.regs[*v as usize],
            _ => panic!("Unsupported operand {:?}", operand)
        }
    }
    fn set(&mut self, operand: &Operand, value: u64) {
        match operand {
            Operand::Imm(v) => {}, // do nothing assume it is r0
            Operand::Reg(v) => {
                self.regs[*v as usize] = value;
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
    
    pub fn run(&mut self) -> StepResult {
        loop {
            let result = self.step();
            if result != StepResult::Continue {
                return result;
            }
        }
    }
    // maybe instead of f64 use Duration but what ever will do for now
    // yeah but idk how to do ins
    pub fn run_for(&mut self, max_time: Duration) -> StepResult {
        const BURST_LENGTH: u32 = 1024;
        let start = now();
        let end = start + max_time.as_secs_f64() * 1000.0;
        while now() < end {
            for _ in 0..BURST_LENGTH {
                let result = self.step();
                if result != StepResult::Continue {
                    return result;
                }
            }
        }
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
            IMM(a, b) => self.set(a, self.get(b)),
            ADD(op1, op2, op3) => {
                self.set(op1, self.get(op2)+self.get(op3));
            },
            RSH(a, b) => self.set(a, self.get(a) >> self.get(b)),
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
            IN(a,b) => todo!(),
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
struct InstBuffer {
    index: usize,
    insts: Vec<Inst>
}
impl InstBuffer {
    #[inline]
    pub fn new(insts: Vec<Inst>) -> Self {
        InstBuffer {
            insts: insts,
            index: 0,
        }
    }
    #[inline]
    pub fn has_next(&self) -> bool {
        self.index < self.insts.len()
    }
    #[inline]
    pub fn advance(&mut self) {
        self.index += 1;
    }
    #[inline]
    pub fn next(&mut self) -> Inst {
        self.advance();
        self.insts[self.index].clone()
    }
    #[inline]
    pub fn current(&self) -> Inst {
        if self.has_next() {
            self.insts[self.index].clone()
        } else{
            Inst::HLT
        }
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn emulate(src: &str) {
    clear_text();
    let toks = lexer::lex(src);
    let program = ast::gen_ast(toks);
    jsprintln!("{:#?}", program);

    let mut host = DeviceHost::new();
    let mut emu = EmulatorState::new(program, host);

    // i can add more insts while you guys do that
    // also please do the too late label thing
    // we need to get some web workers out 👷‍♂️
    let result = emu.run_for(Duration::from_millis(1000));
    if result == StepResult::Continue {
        jsprintln!("Program took too long");
    } else {
        jsprintln!("Program Halted");
    }
    emu.devices.show();

    // for _ in 0..100 {
    //     jsprintln!("{:?}: {:?}", emu.pc, emu.regs);
    //     if emu.step() != StepResult::Continue {
    //         break;
    //     }
    // }
}