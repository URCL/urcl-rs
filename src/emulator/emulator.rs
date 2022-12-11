use wasm_bindgen::prelude::*;
use super::{*, lexer};

struct EmulatorState {
    regs: Vec<i64>,
    heap: Vec<i64>,
    pc: i64,
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn emulate(src: &str) {
    let toks = lexer::lex(src);
    for tok in toks {
        let class = tok.kind.cssClass();
        // TODO: add out_highlight or something
        out_html(&format!("<span class={}>{}</span>", class, tok.str));
    }
}