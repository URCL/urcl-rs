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
    let err = ErrorContext::new();
    let ast = Program::new();
    let buf = TokenBuffer::new(toks);
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

                    "imm" => inst(Inst::MOV(p.get_reg(), p.get_imm())           , &mut p),
                    "mov" => inst(Inst::MOV(p.get_reg(), p.get_op())            , &mut p),
                    "add" => inst(Inst::ADD(p.get_reg(), p.get_op(), p.get_op()), &mut p),
                    "rsh" => inst(Inst::RSH(p.get_reg(), p.get_op())            , &mut p),
                    "lod" => inst(Inst::LOD(p.get_reg(), p.get_mem())           , &mut p),
                    "str" => inst(Inst::STR(p.get_mem(), p.get_op())            , &mut p),
                    "bge" => inst(Inst::BGE(p.get_jmp(), p.get_op(), p.get_op()), &mut p),
                    "nor" => inst(Inst::NOR(p.get_reg(), p.get_op(), p.get_op()), &mut p),
                    "inc" => inst(Inst::INC(p.get_reg(), p.get_op())            , &mut p),
                    "dec" => inst(Inst::DEC(p.get_reg(), p.get_op())            , &mut p),
                    "hlt" => inst(Inst::HLT                                     , &mut p),
                    "sub" => inst(Inst::SUB(p.get_reg(), p.get_op(), p.get_op()), &mut p),
                    "nop" => inst(Inst::NOP                                     , &mut p),
                    "lsh" => inst(Inst::LSH(p.get_reg(), p.get_op())            , &mut p),
                    "out" => inst(Inst::OUT(p.get_port(), p.get_op())           , &mut p),
                    "in"  => inst(Inst::IN (p.get_reg(), p.get_port())          , &mut p),
                    "psh" => inst(Inst::PSH(p.get_op())                         , &mut p),
                    "pop" => inst(Inst::POP(p.get_reg())                        , &mut p),
                    "jmp" => inst(Inst::JMP(p.get_jmp())                        , &mut p),
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

fn inst<'a>(inst: Inst, p: &mut Parser<'a>) {
    p.ast.instructions.push(inst);
    p.assert_done();
}

impl <'a> Parser<'a> {
    fn get_reg(&mut self) -> Operand {
        let (ast, op) = self.get_ast_op();
        match ast {
            AstOp::Reg(_) | AstOp::Unknown => {},
            actual => {
                self.err.error(self.buf.cur(), ErrorKind::InvalidOperandType{
                    expected: "register", actual
                });
            }
        }
        op
    }
    fn get_port(&mut self) -> Operand {
        let (ast, op) = self.get_ast_op();
        match ast {
            AstOp::Reg(_) | AstOp::Port(_) | AstOp::Unknown => {},
            actual => {
                self.err.warn(self.buf.cur(), ErrorKind::InvalidOperandType{
                    expected: "port", actual
                });
            }
        }
        op
    }
    fn get_mem(&mut self) -> Operand {
        let (ast, op) = self.get_ast_op();
        match ast {
            AstOp::Reg(_) | AstOp::Mem(_) | AstOp::Unknown => {},
            actual => {
                self.err.warn(self.buf.cur(), ErrorKind::InvalidOperandType{
                    expected: "memory address", actual
                });
            }
        }
        op
    }
    fn get_jmp(&mut self) -> Operand {
        let (ast, op) = self.get_ast_op();
        match ast {
            AstOp::Reg(_) | AstOp::Label(_) | AstOp::Unknown => {},
            actual => {
                self.err.warn(self.buf.cur(), ErrorKind::InvalidOperandType{
                    expected: "jump target", actual
                });
            }
        }
        op
    }
    fn get_imm(&mut self) -> Operand {
        let (ast, op) = self.get_ast_op();
        match ast {
            AstOp::Reg(_) => {
                self.err.warn(self.buf.cur(), ErrorKind::InvalidOperandType{
                    expected: "immediate", actual: ast
                });
            },
            _ => {}
        }
        op
    }

    fn get_op(&mut self) -> Operand {
        self.get_ast_op().1
    }
    fn trans_op(&mut self, op: &AstOp) -> Operand {
        match op {
            AstOp::Unknown => Operand::Imm(0),
            AstOp::Int(v) => Operand::Imm(*v),
            AstOp::Reg(v) => Operand::Reg(*v),
            AstOp::Mem(v) => Operand::Imm(*v),
            AstOp::Port(v) => Operand::Imm(*v),
            AstOp::Char(v) => Operand::Imm(*v as u64),
            AstOp::String(v) => Operand::Imm(0),
            AstOp::Label(v) => {
                label_tok_to_operand(&self.buf.current(), self)
            },
        }
    }
    fn get_ast_op(&mut self) -> (AstOp, Operand){
        self.buf.advance();
        let current = self.buf.current();
        let ast = match current.kind {
            Kind::Reg(v) => AstOp::Reg(v),
            Kind::Int(v) => AstOp::Int(v as u64),
            Kind::Memory(m) => AstOp::Mem(m),
            Kind::PortNum(v) => AstOp::Port(v),
            Kind::Port => {
                match IOPort::from_str(&current.str[1..].to_uppercase()) {
                    Ok(port) => {AstOp::Port(port as u64)},
                    Err(err) => {
                        self.err.error(&self.buf.current(), ErrorKind::UnknownPort);
                        AstOp::Port(0)
                    }
                }
            }
            Kind::Label  => AstOp::Label(current.str[1..].to_owned()),
            Kind::Char => {
                match self.buf.next().kind {
                    Kind::Text => {
                        if !matches!(self.buf.next().kind, Kind::Char) {
                            self.err.error(&self.buf.current(), ErrorKind::EOFBeforeEndOfString);
                        }
                        AstOp::Char(self.buf.current().str.chars().next().unwrap())
                    }
                    Kind::Escape(c) => {
                        if !matches!(self.buf.next().kind, Kind::Char) {
                            self.err.error(&self.buf.current(), ErrorKind::EOFBeforeEndOfString);
                        }
                        AstOp::Char(c)
                    }
                    _ => {
                        self.err.error(&self.buf.current(), ErrorKind::EOFBeforeEndOfString);
                        AstOp::Char('\x00')
                    },
                }
            }
            Kind::String => {
                let mut text = String::new();
                while self.buf.has_next() {match self.buf.next().kind {
                    Kind::String => break,
                    Kind::Text => text += self.buf.cur().str,
                    Kind::Escape(c) => text.push(c),
                    _ => {
                        self.err.error(&self.buf.current(), ErrorKind::EOFBeforeEndOfString);
                        break;
                    }
                }}


                AstOp::String(text)
            }
            Kind::EOF | Kind::LF => {
                self.err.error(&self.buf.current(), ErrorKind::NotEnoughOperands);
                AstOp::Unknown
            }
            _ => {
                self.err.error(&self.buf.current(), ErrorKind::InvalidOperand);
                AstOp::Unknown
            }
        };
        let op = self.trans_op(&ast);
        (ast, op)
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
    pub labels: HashMap<String, Label>,
    pub memory: Vec<u64>,
}

impl Program {
    pub fn new() -> Self {
        Program { headers: Headers::new(), instructions: Vec::new(), labels: HashMap::new(), memory: Vec::new() }
    }
}

#[derive(Debug, Clone)] // cant copy because of the String
pub enum AstOp {
    Unknown,
    Int(u64),
    Reg(u64),
    Mem(u64),
    Port(u64),
    Char(char),
    String(String),
    Label(String),
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
