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
    clear_span();
    let toks = lexer::lex(src);
    for tok in toks {
        let class = tok.kind.cssClass();
        // TODO: add out_highlight or something
        out_span(tok.str, class);
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn output_highlight_span(src: &str) {
    clear_span();
    let toks = lexer::lex(src);
    for tok in toks {
        if tok.str == "\n" {
            out_lf();
        } else {
            let class = tok.kind.cssClass();
            out_span(tok.str, class);
        }
    }
}
