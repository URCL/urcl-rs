use devices::DeviceHost;
use std::{rc::Rc, time::Duration};

use crate::emulator::ast::Parser;
use wasm_bindgen::prelude::*;

use super::{
    ast::{self, Inst, Operand, Program},
    lexer, *,
};

#[derive(Debug)]
pub enum EmulatorErrorKind {
    StackOverflow,
    StackUnderflow,
}

impl<'a> std::fmt::Display for EmulatorErrorKind {
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
        Stack { data, sp: size as i64 - 1, size }
    }

    fn push(&mut self, data: u64) -> Result<(), EmulatorError> {
        if self.sp > 0 {
            self.data[self.sp as usize] = data;
            self.sp -= 1;
            Ok(())
        } else {
            Err(EmulatorError(Some(EmulatorErrorKind::StackOverflow)))
        }
    }
    fn pop(&mut self) -> Result<u64, EmulatorError> {
        if self.sp < self.size as i64 - 1 {
            self.sp += 1;
            Ok(self.data[self.sp as usize])
        } else {
            Err(EmulatorError(Some(EmulatorErrorKind::StackUnderflow)))
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq)]
pub enum StepResult {
    Continue,
    HLT,
    Input,
    Error,
}

pub const PC: u64 = u64::MAX;
pub const SP: u64 = u64::MAX - 1;

fn does_overflow(a: u64, b: u64) -> bool {
    match a.checked_add(b) {
        Some(_) => false,
        None => true,
    }
}

// you cant bindgen impls i dont think
#[wasm_bindgen]
#[allow(dead_code)]
impl EmulatorState {
    fn new(program: Program, devices: DeviceHost) -> Self {
        let regs = vec![0; program.headers.minreg as usize];
        let heap = vec![0; (program.headers.minheap /* + program.headers.minstack*/) as usize];
        EmulatorState {
            regs,
            heap,
            stack: Stack::new(program.headers.minstack as usize),
            pc: 0,
            program,
            devices,
            error: EmulatorError::new(),
        }
    }

    pub fn run(&mut self) -> StepResult {
        loop {
            let result = self.step();
            match result {
                StepResult::Continue => (),
                StepResult::Error => return result,
                _ => {
                    self.devices.show();
                    return result;
                }
            }
        }
    }
    fn run_for(&mut self, max_time: Duration) -> StepResult {
        return self.run_for_ms(max_time.as_secs_f64() * 1000.0);
    }
    pub fn show(&mut self) {
        clear_text();
        self.devices.show();
        jsprintln!("Regs: {:?},\nMem: {:?},\nStack: {:?}", self.regs, self.heap, self.stack.data);
    }

    pub fn run_for_ms(&mut self, max_time_ms: f64) -> StepResult {
        const BURST_LENGTH: u32 = 1024;
        let start = now();
        let end = start + max_time_ms;
        while now() < end {
            for _ in 0..BURST_LENGTH {
                let result = self.step();
                match result {
                    StepResult::Continue => (),
                    StepResult::Error => return result,
                    _ => {
                        self.devices.show();
                        return result;
                    }
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
        let Some(inst) = self.program.instructions.get(self.pc) else {
            return StepResult::HLT
        };

        macro_rules! get {
            ($operand:expr) => {
                match $operand {
                    Operand::Imm(v) => *v,
                    Operand::Reg(v) => match *v {
                        PC => self.pc as u64,
                        SP => self.stack.sp as u64,
                        0  => 0,
                        _  => self.regs[*v as usize - 1],
                    },
                    _ => panic!("Unsupported operand {:?}", $operand),
                }
            };
        }
        macro_rules! set {
            ($operand:expr, $value:expr) => {
                match $operand {
                    Operand::Imm(_) => {} // do nothing assume it is r0
                    Operand::Reg(v) => match *v {
                        PC => self.pc = $value as usize,
                        SP => self.stack.sp = $value as i64,
                        0  => {},
                        _  => self.regs[*v as usize - 1] = $value,
                    },
                    _ => panic!("Unsupported target operand {:?}", $operand),
                }
            };
        }

        macro_rules! get_mem {
            ($index:expr) => {
                if $index < self.program.headers.minheap {
                    self.heap[$index as usize]
                } else {
                    self.stack.data[($index - self.program.headers.minheap) as usize]
                }
            };
        }
        macro_rules! set_mem {
            ($index:expr, $value:expr) => {
                if $index < self.program.headers.minheap {
                    self.heap[$index as usize] = $value
                } else {
                    self.stack.data[($index - self.program.headers.minheap) as usize] = $value
                }
            };
        }

        macro_rules! getm {
            ($operand:expr) => {
                if get!($operand) < self.program.headers.minheap {
                    self.heap[get!($operand) as usize]
                } else {
                    self.stack.data[(get!($operand) - self.program.headers.minheap) as usize]
                }
            };
        }
        macro_rules! setm {
            ($operand:expr, $value:expr) => {
                if get!($operand) < self.program.headers.minheap {
                    self.heap[get!($operand) as usize] = $value
                } else {
                    self.stack.data[(get!($operand) - self.program.headers.minheap) as usize] = $value
                }
            };
        }

        macro_rules! insts {
            (@pat($name:ident); $($raw:ident)*) => {
                Inst::$name($($raw),*)
            };
            (@pat($name:ident) ($($($raw:ident$(: $_type_raw:ty)?)? $([$mem:ident$(: $_type_mem:ty)?])?),*)) => {
                insts!(@pat($name); $($($raw)?)? $($($mem)?)?)
            };
            (@pat($name:ident)) => {
                Inst::$name
            };
            (@read) => {};
            (@read [$name:ident$(: $type:ty)?]$(, $($rest:tt)*)?) => {
                #[allow(unused_variables)]
                let $name = getm!($name) $(as $type)?;
                insts!(@read $($($rest)*)?)
            };
            (@read $name:ident$(: $type:ty)?$(, $($rest:tt)*)?) => {
                #[allow(unused_variables)]
                let $name = get!($name) $(as $type)?;
                insts!(@read $($($rest)*)?)
            };
            (@assign; $body:expr) => {
                $body
            };
            (@assign $to:ident; $body:expr) => {{
                let value = $body as u64;
                set!($to, value)
            }};
            (@assign [$to:ident]; $body:expr) => {{
                let value = $body as u64;
                setm!($to, value)
            }};
            (
                $($name:ident$(($($ops:tt)*))? $(; $assign:tt)? => $body:expr$(,)?)*
            ) => {
                match inst {
                    $(

                        insts!(@pat($name) $(($($ops)*))?) => {
                            insts!(@assign $($assign)?; {
                                $(insts!(@read $($ops)*);)?
                                $body
                            })
                        }
                    )*
                }
            };
        }

        macro_rules! branch {
            ($dest:ident $(if $cond:expr)?) => {
                match () {
                    () $(if $cond)? => self.pc = $dest - 1,
                    #[allow(unreachable_patterns)]
                    () => (),
                }
            }
        }

        macro_rules! SET {
            ($cond:expr) => {
                if $cond {
                    u64::MAX
                } else {
                    0
                }
            };
        }

        insts! {
            NOP => {},
            HLT => return StepResult::HLT,

            PSH(a) => {
                if let Err(err) = self.stack.push(a) {
                    self.error = err;
                }
            },
            POP(a); a => {
                match self.stack.pop() {
                    Ok(v) => v,
                    Err(err) => {
                        self.error = err;
                        a
                    },
                }
            },
            CAL(a: usize) => {
                if let Err(err) = self.stack.push(self.pc as u64) {
                    self.error = err;
                }
                branch!(a)
            },
            RET => {
                match self.stack.pop().map(|v| v as usize) {
                    Ok(v) => branch!(v),
                    Err(err) => self.error = err,
                }
            },

            IN(a, b) => todo!(),
            OUT(a, b) => self.devices.out(a, b),

            JMP(a: usize) => branch!(a),
            BRG(a: usize, b, c) => branch!(a if b > c),
            BGE(a: usize, b, c) => branch!(a if b >= c),
            BRL(a: usize, b, c) => branch!(a if b < c),
            BLE(a: usize, b, c) => branch!(a if b <= c),

            BRE(a: usize, b, c) => branch!(a if b == c),
            BNE(a: usize, b, c) => branch!(a if b != c),
            BRZ(a: usize, b) => branch!(a if b == 0),
            BNZ(a: usize, b) => branch!(a if b != 0),
            BRC(a: usize, b, c) => branch!(a if does_overflow(b, c)),
            BNC(a: usize, b, c) => branch!(a if !does_overflow(b, c)),

            SBRG(a: usize, b: i64, c: i64) => branch!(a if b > c),
            SBGE(a: usize, b: i64, c: i64) => branch!(a if b >= c),
            SBRL(a: usize, b: i64, c: i64) => branch!(a if b < c),
            SBLE(a: usize, b: i64, c: i64) => branch!(a if b <= c),

            BEV(a: usize, b) => branch!(a if b&1 == 0),
            BOD(a: usize, b) => branch!(a if b&1 == 1),
            BRP(a: usize, b: i64) => branch!(a if b >= 0),
            BRN(a: usize, b: i64) => branch!(a if b < 0),

            MOV(a, b); a => b,
            STR(a, b); [a] => b,
            CPY(a, [b]); [a] => b,
            LOD(a, [b]); a => b,
            LLOD(a, b, c); a => get_mem!(b + c),
            LSTR(a, b, c) => set_mem!(a + b, c),

            ADD(a, b, c); a => b + c,
            SUB(a, b, c); a => b - c,
            INC(a, b); a => b + 1,
            DEC(a, b); a => b - 1,

            RSH(a, b); a => b >> 1,
            LSH(a, b); a => b << 1,
            SRS(a, b: i64); a => b >> 1,

            BSR(a, b, c); a => b >> c,
            BSL(a, b, c); a => b << c,
            BSS(a, b: i64, c: i64); a => b >> c,

            OR(a, b, c); a => b | c,
            NOR(a, b, c); a => !(b | c),
            AND(a, b, c); a => b & c,
            NAND(a, b, c); a => !(b & c),
            XOR(a, b, c); a => b ^ c,
            XNOR(a, b, c); a => !(b ^ c),

            NOT(a, b); a => !b,
            NEG(a, b: i64); a => -b,
            ABS(a, b: i64); a => b.abs(),

            MLT(a, b, c); a => b * c,
            DIV(a, b, c); a => b / c,
            SDIV(a, b: i64, c: i64); a => b / c,
            MOD(a, b, c); a => b % c,

            SETE(a, b, c); a => SET!(b == c),
            SETNE(a, b, c); a => SET!(b != c),
            SETC(a, b, c); a => SET!(does_overflow(b, c)),
            SETNC(a, b, c); a => SET!(!does_overflow(b, c)),

            SETG(a, b, c); a => SET!(b > c),
            SETGE(a, b, c); a => SET!(b >= c),
            SETL(a, b, c); a => SET!(b < c),
            SETLE(a, b, c); a => SET!(b <= c),
            SSETG(a, b: i64, c: i64); a => SET!(b > c),
            SSETGE(a, b: i64, c: i64); a => SET!(b >= c),
            SSETL(a, b: i64, c: i64); a => SET!(b < c),
            SSETLE(a, b: i64, c: i64); a => SET!(b <= c),
        }

        self.pc += 1;

        match &self.error {
            EmulatorError(Some(err)) => {
                jsprintln!(
                    "<span class=\"error\">Emulator Error: {} at line {}</span>",
                    err,
                    self.program.debug.pc_to_line_start[self.pc-1]
                );
                StepResult::Error
            }
            EmulatorError(None) => StepResult::Continue,
        }
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn emulate(src: String) -> Option<EmulatorState> {
    // wifi died
    let src = Rc::from(src);
    clear_text();
    let toks = lexer::lex(&src);

    let Parser {
        ast: program, err, ..
    } = ast::gen_ast(toks, src.clone());
    jsprintln!("{}", err.to_string(&src));
    if err.has_error() {
        return None;
    }

    let host = DeviceHost::new();
    let emu = EmulatorState::new(program, host);

    return Some(emu);
}
