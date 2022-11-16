use std::fmt::format;

use wasm_bindgen::prelude::*;
use super::{*, lexer::*};

struct EmulatorState {
    regs: Vec<i64>,
    heap: Vec<i64>,
    pc: i64,
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn emulate(src: &str) {
    jsprintln!("emulate called");
    let toks = tokenise(src);
    for tok in toks {
        match tok {
            Token::Label(_) => jsprintln!("Label: {:#?}", tok),
            Token::String(_) => jsprintln!("String: {:#?}", tok),
            Token::Char(_) => jsprintln!("Char: {:#?}", tok),
            _ => {}
        }
    }
}