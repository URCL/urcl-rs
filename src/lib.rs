use wasm_bindgen::prelude::*;
mod emulator;

#[wasm_bindgen(raw_module="../script.js")]
extern {
    pub fn out_text(text: &str);
    pub fn out_graphics(x: u64, y: u64, colour: u64);
    pub fn in_text() -> String;
    pub fn out_err(text: &str);
}

#[wasm_bindgen()]
extern {
    pub fn log(s: &str);
}