use super::{*, lexer2::{Token, Kind, UToken}};

struct TokenBuffer<'a> {
    index: usize,
    toks: Vec<UToken<'a>>
}
impl <'a> TokenBuffer<'a> {
    #[inline]
    pub fn new() -> Self {
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
    pub fn next(&mut self) -> UToken<'a> {
        self.index += 1;
        self.toks[self.index].clone()
    }
    #[inline]
    pub fn current(&self) -> UToken<'a> {
        self.toks[self.index].clone()
    }
}

pub fn gen_ast<'a>(toks: Vec<UToken<'a>>) {
    let mut token_buffer: TokenBuffer = TokenBuffer::new();
}


pub enum Node<'a> {
    AddNote {
        opperands: Vec<UToken<'a>>
    },
    ImmNode {
        opperands: Vec<UToken<'a>>
    },
}