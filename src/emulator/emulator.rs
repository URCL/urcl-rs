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
    let toks = tokenise(src);
    for tok in toks {
        jsprintln!("{:#?}", tok)
    }
}