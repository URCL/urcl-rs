use std::{collections::HashMap, str::FromStr};

use super::{*, lexer::{Token, Kind, UToken}, errorcontext::{ErrorContext, ErrorKind}, devices::IOPort};

struct TokenBuffer<'a> {
    index: usize,
    toks: Vec<UToken<'a>>
}
impl <'a> TokenBuffer<'a> {
    #[inline]
    pub fn new(toks: Vec<UToken<'a>>) -> Self {
        TokenBuffer {
            toks: toks,
            index: 0,
        }
    }
    #[inline]
    pub fn has_next(&self) -> bool {
        self.index < self.toks.len()
    }
    #[inline]
    pub fn advance(&mut self) {
        self.index += 1;
        while self.current().kind == Kind::White || self.current().kind == Kind::Comment {
            self.index += 1;
        }
    }
    #[inline]
    pub fn next(&mut self) -> UToken<'a> {
        self.advance();
        self.toks[self.index].clone()
    }
    #[inline]
    pub fn current(&self) -> UToken<'a> {
        if self.has_next() {
            self.toks[self.index].clone()
        } else{
            Token {kind: Kind::EOF, str: ""}
        }
    }
    pub fn cur(&self) -> &UToken<'a> {
        if self.has_next() {
            &self.toks[self.index]
        } else {
            self.toks.last().unwrap()
        }
    }
}

pub struct Parser<'a> {
    buf: TokenBuffer<'a>,
    pub err: ErrorContext<'a>,
    pub ast: Program
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

pub fn gen_ast<'a>(toks: Vec<UToken<'a>>) -> Parser {
    let mut err = ErrorContext::new();
    let mut ast = Program::new();
    let mut buf = TokenBuffer::new(toks);
    let mut p = Parser {buf, err, ast};

    while p.buf.has_next() {
        match p.buf.current().kind {
            Kind::Name => {
                match p.buf.current().str.to_lowercase().as_str() {
                    "bits" => {
                        p.ast.headers.bits = match p.buf.next().kind { Kind::Int(v) => v as u64, _ => match p.buf.next().kind {Kind::Int(v) => v as u64, _ => continue} };
                        p.buf.advance();
                    },
                    "minreg" => {
                        p.ast.headers.minreg = match p.buf.next().kind {Kind::Int(v) => v as u64, _ => {continue;}};
                        p.buf.advance();
                    },
                    "minheap" => {
                        p.ast.headers.minheap = match p.buf.next().kind {Kind::Int(v) => v as u64, _ => {continue;}};
                        p.buf.advance();
                    },
                    "minstack" => {
                        p.ast.headers.minstack = match p.buf.next().kind {Kind::Int(v) => v as u64, _ => {continue;}};
                        p.buf.advance();
                    },

                    "imm" => {let inst = Inst::MOV(p.get_reg(), p.get_imm())            ; p.inst(inst)}, // TODO: maybe check for strictly imm
                    "mov" => {let inst = Inst::MOV(p.get_reg(), p.get_op())             ; p.inst(inst)},
                    "add" => {let inst = Inst::ADD(p.get_reg(), p.get_op(), p.get_op()) ; p.inst(inst)},
                    "rsh" => {let inst = Inst::RSH(p.get_reg(), p.get_op())             ; p.inst(inst)},
                    "lod" => {let inst = Inst::LOD(p.get_reg(), p.get_mem())            ; p.inst(inst)},
                    "str" => {let inst = Inst::STR(p.get_mem(), p.get_op())             ; p.inst(inst)},
                    "bge" => {let inst = Inst::BGE(p.get_jmp(), p.get_op(), p.get_op()) ; p.inst(inst)},
                    "nor" => {let inst = Inst::NOR(p.get_reg(), p.get_op(), p.get_op()) ; p.inst(inst)},
                    "inc" => {let inst = Inst::INC(p.get_reg(), p.get_op())             ; p.inst(inst)},
                    "dec" => {let inst = Inst::DEC(p.get_reg(), p.get_op())             ; p.inst(inst)},
                    "hlt" => {let inst = Inst::HLT                                      ; p.inst(inst)},
                    "sub" => {let inst = Inst::SUB(p.get_reg(), p.get_op(), p.get_op()) ; p.inst(inst)},
                    "nop" => {let inst = Inst::NOP                                      ; p.inst(inst)},
                    "lsh" => {let inst = Inst::LSH(p.get_reg(), p.get_op())             ; p.inst(inst)},
                    "out" => {let inst = Inst::OUT(p.get_port(), p.get_op())            ; p.inst(inst)},
                    "in"  => {let inst = Inst::IN(p.get_reg(), p.get_port())            ; p.inst(inst)},
                    "psh" => {let inst = Inst::PSH(p.get_op())                          ; p.inst(inst)},
                    "pop" => {let inst = Inst::POP(p.get_reg())                         ; p.inst(inst)},
                    "jmp" => {let inst = Inst::JMP(p.get_jmp())                         ; p.inst(inst)},
                    _ => { p.err.error(&p.buf.current(), ErrorKind::UnknownInstruction); p.buf.advance(); },
                }
            },
            Kind::Label => {
                match p.ast.labels.get(p.buf.current().str) {
                    Some(Label::Defined(_)) => p.err.error(&p.buf.current(), ErrorKind::DuplicatedLabelName),
                    Some(Label::Undefined(v)) => {
                        let label_name = p.buf.current().str; let pc = p.ast.instructions.len();
                        for i in v.iter() {
                            p.ast.instructions[*i] = match &p.ast.instructions[*i] {
                                Inst::PSH(a) => Inst::PSH(a.clone().transform_label(label_name, pc)),
                                Inst::JMP(a) => Inst::JMP(a.clone().transform_label(label_name, pc)),
                                Inst::MOV(a, b) => Inst::MOV(a.clone(), b.clone().transform_label(label_name, pc)),
                                Inst::IN (a, b) => Inst::IN(a.clone(), b.clone().transform_label(label_name, pc)),
                                Inst::OUT(a, b) => Inst::OUT(a.clone(), b.clone().transform_label(label_name, pc)),
                                Inst::INC(a, b) => Inst::INC(a.clone(), b.clone().transform_label(label_name, pc)),
                                Inst::DEC(a, b) => Inst::DEC(a.clone(), b.clone().transform_label(label_name, pc)),
                                Inst::LSH(a, b) => Inst::LSH(a.clone(), b.clone().transform_label(label_name, pc)),
                                Inst::RSH(a, b) => Inst::RSH(a.clone(), b.clone().transform_label(label_name, pc)),
                                Inst::LOD(a, b) => Inst::LOD(a.clone(), b.clone().transform_label(label_name, pc)),
                                Inst::STR(a, b) => Inst::STR(a.clone().transform_label(label_name, pc), b.clone()),
                                Inst::ADD(a, b, c) => Inst::ADD(a.clone(), b.clone().transform_label(label_name, pc), c.clone().transform_label(label_name, pc)),
                                Inst::SUB(a, b, c) => Inst::SUB(a.clone(), b.clone().transform_label(label_name, pc), c.clone().transform_label(label_name, pc)),
                                Inst::NOR(a, b, c) => Inst::NOR(a.clone(), b.clone().transform_label(label_name, pc), c.clone().transform_label(label_name, pc)),
                                Inst::BGE(a, b, c) => Inst::BGE(a.clone().transform_label(label_name, pc), b.clone().transform_label(label_name, pc), c.clone().transform_label(label_name, pc)),
                                _ => continue,
                            }
                        }
                        p.ast.labels.insert(p.buf.current().str.to_string(), Label::Defined(p.ast.instructions.len()));
                    },
                    None => { p.ast.labels.insert(p.buf.current().str.to_string(), Label::Defined(p.ast.instructions.len())); },
                }
                p.buf.advance();
            },
            Kind::White | Kind::Comment | Kind::LF | Kind::Char | Kind::String => p.buf.advance(),
            _ => { logprintln!("Unhandled token type: {:#?}", p.buf.current()); p.buf.advance(); },
        }
    }

    p
}



impl <'a> Parser<'a> {
    fn inst(&mut self, inst: Inst) {
        self.ast.instructions.push(inst);
        self.assert_done();
    }

    fn get_reg(&mut self) -> Operand {
        let operand = self.get_op();
        match operand {
            Operand::Reg(_) => {},
            _ => {
                self.err.error(self.buf.cur(), ErrorKind::InvalidOperandType);
            }
        }
        operand
    }
    fn get_port(&mut self) -> Operand {
        self.get_op() // TODO: check if port
    }
    fn get_mem(&mut self) -> Operand {
        self.get_op() // TODO: check if memory
    }
    fn get_jmp(&mut self) -> Operand {
        self.get_op() // TODO: check if jump target
    }
    fn get_imm(&mut self) -> Operand {
        let op = self.get_op();
        match op {
            Operand::Reg(_) => {
                self.err.error(self.buf.cur(), ErrorKind::InvalidOperandType);
            }
            _ => {},
        }
        op
    }

    fn get_op(&mut self) -> Operand {
        self.buf.advance();
        let current = self.buf.current();
        match current.kind {
            Kind::Reg(v) => Operand::Reg(v),
            Kind::Int(v) => Operand::Imm(v as u64),
            Kind::PortNum(v) => Operand::Imm(v),
            Kind::Port => {
                match IOPort::from_str(&current.str[1..].to_uppercase()) {
                    Ok(port) => {Operand::Imm(port as u64)},
                    Err(err) => {
                        self.err.error(&self.buf.current(), ErrorKind::UnknownPort);
                        Operand::Imm(0)
                    }
                }
            }
            Kind::Label  => label_tok_to_operand(&self.buf.current(), self),
            _ => {
                Operand::Imm(0)
            }
        }
    }
    fn assert_done(&mut self) {
        self.buf.advance();
        match self.buf.current().kind {
            Kind::LF |  Kind::EOF => {},
            _ => {
                self.err.error(&self.buf.current(), ErrorKind::ToManyOperands);
                while match self.buf.current().kind {Kind::LF |  Kind::EOF => false, _ => true} {
                    self.buf.advance()
                } 
            }
        }
    }
}


fn get_operand(p: &mut Parser) -> Option<Operand> {
    match p.buf.current().kind {
        Kind::Reg(v) => Some(Operand::Reg(v)),
        Kind::Int(v) => Some(Operand::Imm(v as u64)),
        Kind::PortNum(v) => Some(Operand::Imm(v)),
        Kind::Port => {
            match IOPort::from_str(&p.buf.current().str[1..].to_uppercase()) {
                Ok(port) => {Some(Operand::Imm(port as u64))},
                Err(err) => {
                    p.err.error(&p.buf.current(), ErrorKind::UnknownPort);
                    None
                }
            }
        }
        Kind::Label  => Some(label_tok_to_operand(&p.buf.current(), p)),
        _ => None
    }
}

#[derive(Debug, PartialEq)]
pub enum Label {
    Undefined(Vec<usize>),
    Defined(usize),
}

fn label_tok_to_operand<'a>(tok: &UToken<'a>, p: &mut Parser) -> Operand {
    if (*tok).kind != Kind::Label {return Operand::Imm(0);}

    match p.ast.labels.get(tok.str) {
        Some(Label::Undefined(v)) => {
            let mut a = v.clone();
            a.push(p.ast.instructions.len());
            p.ast.labels.insert((*tok).str.to_string(), Label::Undefined(a));
            Operand::Label(tok.str.to_string())
        },
        Some(Label::Defined(v)) => Operand::Imm(*v as u64),
        None => {
            let mut a = Vec::new();
            a.push(p.ast.instructions.len());
            p.ast.labels.insert((*tok).str.to_string(), Label::Undefined(a));
            Operand::Label(tok.str.to_string())
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub headers: Headers,
    pub instructions: Vec<Inst>,
    pub labels: HashMap<String, Label>
}

impl Program {
    pub fn new() -> Self {
        Program { headers: Headers::new(), instructions: Vec::new(), labels: HashMap::new() }
    }
}

#[derive(Debug, Clone)] // cant copy because of the String
pub enum Operand {
    Imm(u64),
    Reg(u64),// it gets changed to immediates, try it out
    Mem(u64),
    Label(String),
}

impl Operand {
    pub fn transform_label(self, label: &str, pc: usize) -> Self {
        match self {
            Operand::Label(ref l) => if l == label {Operand::Imm(pc as u64)} else {self}
            _ => self,
        }
    }
}

// kind (imm, reg, mem, label) 1 byte
// start: 4/8bytes, capacity: 4/8bytes, length: 4/8bytes
// 8*4 = 32 bytes
// but it only has to be 16 bytes
// or even better, we could pack the bytes but that has other diadvantages
// unless... we put the opcodes in a sepperate buffer like this
// but idk what that will do for performance it could make it better it could not matter at all
// we should benchmark our different options
// cant we get it working and then optimize
struct EmuProgram {
    opcodes: Vec<Opcode>, // only 1 byte per instruction instead of the 8 bytes because we don't need to allign it with the immediate 
    // oh right the operands aren't just emmediates... 🤔
    // they can also be registers
    // maybe should just try and se how fast your current implementation is 😌
    immediates: Vec<u64>
}

enum Opcode {}

#[derive(Debug)]
pub struct Headers {
    pub bits: u64,
    pub minheap: u64,
    pub minstack: u64,
    pub minreg: u64
}

impl Headers {
    pub fn new() -> Self {
        Headers { bits: 8, minheap: 16, minstack: 16, minreg: 8 } // replace all r0 with 0
    }
}

#[derive(Debug, Clone)]
pub enum Inst {
    IMM(Operand, Operand),
    ADD(Operand, Operand, Operand),
    RSH(Operand, Operand),
    LOD(Operand, Operand),
    STR(Operand, Operand),
    BGE(Operand, Operand, Operand),
    NOR(Operand, Operand, Operand),
    MOV(Operand, Operand),
    INC(Operand, Operand),
    DEC(Operand, Operand),
    OUT(Operand, Operand),
    IN(Operand, Operand),
    HLT,
    
    PSH(Operand),
    POP(Operand),
    JMP(Operand),
    SUB(Operand, Operand, Operand),
    NOP,
    LSH(Operand, Operand),
}


// pub trait Port {
//     fn port_out(&mut self, data: u64);
//     fn port_in(&mut self) -> u64;
// }
