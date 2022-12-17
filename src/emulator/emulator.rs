use wasm_bindgen::prelude::*;
use super::{*, lexer, ast::{self, Inst}};
struct EmulatorState {
    regs: Vec<i64>,
    heap: Vec<i64>,
    pc: i64,
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
    let toks =  lexer::lex(src);
    let program = ast::gen_ast(toks);
    let mut regs: Vec<u64> = Vec::with_capacity(program.headers.minreg as usize);

}