use wasm_bindgen::prelude::*;
use super::{*, lexer};

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