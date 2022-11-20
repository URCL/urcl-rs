use super::{*, lexer::Token};


struct TokenBuffer {
    index: usize,
    toks: Vec<Token>
}
impl TokenBuffer {
    #[inline]
    pub fn new() -> TokenBuffer {
        TokenBuffer {
            toks: vec![],
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
    pub fn next(&mut self) -> Token {
        self.index += 1;
        self.toks[self.index].clone()
    }
    #[inline]
    pub fn current(&self) -> Token {
        self.toks[self.index].clone()
    }
}

pub fn gen_ast(toks: Vec<Token>) {
    let mut token_buffer: TokenBuffer = TokenBuffer::new();
}


pub enum Node {
    AddNote {
        opperands: Vec<Token>
    },
    ImmNode {
        opperands: Vec<Token>
    },
}