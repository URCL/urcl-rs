use wasm_bindgen::prelude::*;
use super::{*, lexer, ast};

struct EmulatorState {
    regs: Vec<i64>,
    heap: Vec<i64>,
    pc: i64,
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn emulate(src: &str) {
    clear_text();
    out_text(format!("{:#?}", ast::gen_ast(lexer::lex(src))).as_str());
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn output_highlight_span(src: &str) {
    clear_span();
    let toks = lexer::lex(src);
    for tok in toks {
        let class = tok.kind.css_class();
        out_span(tok.str, class);
    }
    out_span("\n\n", "");
}
