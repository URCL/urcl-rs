use std::collections::HashMap;

use super::{*, lexer::{Token, Kind, UToken}};

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
    }
    #[inline]
    pub fn next(&mut self) -> UToken<'a> {
        self.index += 1;
        self.toks[self.index].clone()
    }
    #[inline]
    pub fn current(&self) -> UToken<'a> {
        self.toks[self.index].clone()
    }
}

pub fn gen_ast<'a>(toks: Vec<UToken<'a>>) -> Program {
    let mut ret = Program::new();
    let mut buf = TokenBuffer::new(toks);

    while buf.has_next() {
        match buf.current().kind {
            Kind::White => buf.advance(),
            _ => panic!("Unhandled token type"),
        }
    }

    ret
}


pub struct Program {
    headers: Headers,
    instructions: Vec<Inst>,
    labels: HashMap<&'static str, u64>
}

impl Program {
    pub fn new() -> Self {
        Program { headers: Headers::new(), instructions: Vec::new(), labels: HashMap::new() }
    }
}

pub enum Operand {
    Imm(u64),
    Reg(u64)
}

pub struct Headers {
    bits: u64,
    minheap: u64,
    minstack: u64,
    minram: u64,
    minreg: u64
}

impl Headers {
    pub fn new() -> Self {
        Headers { bits: 8, minheap: 16, minstack: 16, minram: 16, minreg: 8 }
    }
}

pub enum Inst {
    IMM(Operand, Operand),
    ADD(Operand, Operand, Operand),
    RSH(Operand, Operand),
    LOD(Operand, Operand),
    STR(Operand, Operand),
    BGE(Operand, Operand, Operand),
    NOR(Operand, Operand, Operand),
}