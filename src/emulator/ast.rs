use super::{*, lexer::Token};


struct TokenBuffer {
    index: usize,
    toks: Vec<Token>
}
impl TokenBuffer {
    pub fn has_next(&self) -> bool {
        self.index < self.toks.len()
    }
    pub fn advance(&mut self) {
        self.index += 1;
    }
    pub fn next(&mut self) -> Token {
        self.index += 1;
        self.toks[self.index].clone()
    }
    pub fn current(&self) -> Token {
        self.toks[self.index].clone()
    }
}

pub fn gen_ast(toks: Vec<Token>) {
    
}


pub enum Node {
    AddNote {
        opperands: Vec<Token>
    },
    ImmNode {
        opperands: Vec<Token>
    },
}