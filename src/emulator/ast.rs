use std::collections::HashMap;

use super::{*, lexer::{Token, Kind, UToken}, errorcontext::ErrorContext};

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
        while self.current().kind == Kind::White {
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
}

struct Parser<'a> {
    buf: TokenBuffer<'a>,
    err: ErrorContext,
    ast: Program
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

pub fn gen_ast<'a>(toks: Vec<UToken<'a>>) -> Program {
    let mut err = ErrorContext::new();
    let mut ast = Program::new();
    let mut buf = TokenBuffer::new(toks);
    let mut p = Parser {buf, err, ast};

    while p.buf.has_next() {
        match p.buf.current().kind {
            Kind::Name => {
                match p.buf.current().str.to_lowercase().as_str() {
                    "imm" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        
                        p.ast.instructions.push(
                            Inst::IMM(op1, op2)
                        );
                        p.buf.advance();
                    },
                    "mov" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::MOV(op1, op2)
                        );
                        p.buf.advance();
                    },
                    "add" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        let op3 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::ADD(op1, op2, op3)
                        );
                        p.buf.advance();
                    },
                    "rsh" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::RSH(op1, op2)
                        );
                        p.buf.advance();
                    },
                    "lod" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Memory(v) => Operand::Mem(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::LOD(op1, op2)
                        );
                        p.buf.advance();
                    },
                    "str" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Memory(v) => Operand::Mem(v as u64), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::STR(op1, op2)
                        );
                        p.buf.advance();
                    },
                    "bge" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), Kind::Label => label_to_operand(&p.buf.current(), &mut p), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        let op3 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        
                        p.ast.instructions.push(
                            Inst::BGE(op1, op2, op3)
                        );

                        p.buf.advance();
                    },
                    "nor" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        let op3 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::NOR(op1, op2, op3)
                        );
                        p.buf.advance();
                    },
                    "hlt" => {
                        p.ast.instructions.push(
                            Inst::HLT
                        );
                        p.buf.advance();
                    },
                    "bits" => {
                        p.buf.advance();
                        p.ast.headers.bits = match p.buf.next().kind {Kind::Int(v) => v as u64, _ => {continue;}};
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
                    _ => { jsprintln!("Unhandled name: {:#?}", p.buf.current().str); p.buf.advance(); },
                }
            },
            Kind::Label => {
                match p.ast.labels.get(p.buf.current().str) {
                    Some(Label::Defined(_)) => jsprintln!("Redefined label: {}", p.buf.current().str),
                    Some(Label::Undefined(v)) => {
                        // for i in v.iter() {
                        //     p.ast.instructions[*i]
                        // }
                        jsprintln!("Defined label {} too late lol I didnt impl that", p.buf.current().str);
                    },
                    None => { p.ast.labels.insert(p.buf.current().str.to_string(), Label::Defined(p.ast.instructions.len())); },
                }
                p.buf.advance();
            },
            Kind::White | Kind::Comment | Kind::LF => p.buf.advance(),
            _ => { logprintln!("Unhandled token type: {:#?}", p.buf.current());  p.buf.advance(); },
        }
    }

    p.ast
}

// impl Parser { // bram if i commit this can i go to sleep
    // fn operand(&mut self) -> Option<Operand> {
    //      let op = self.buf.current().
    // }
// }

#[derive(Debug, PartialEq)]
pub enum Label {
    Undefined(Vec<usize>),
    Defined(usize),
}

fn label_to_operand<'a>(tok: &UToken<'a>, p: &mut Parser) -> Operand {
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

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(u64),
    Reg(u64),
    Mem(u64),
    Label(String),
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
        Headers { bits: 8, minheap: 16, minstack: 16, minreg: 8 }
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
    HLT,
}
