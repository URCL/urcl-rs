use wasm_bindgen::prelude::*;
mod emulator;

extern crate console_error_panic_hook;
use std::panic;


#[wasm_bindgen(raw_module="../script.js")]
extern {
    pub fn out_graphics(x: u64, y: u64, colour: u64);

    pub fn clear_text();
    pub fn in_text() -> String;
    pub fn out_text(text: &str);

    pub fn out_err(text: &str);
    
    pub fn out_span(text: &str, class_name: &str);
    pub fn clear_span();
    
    pub fn now() -> f6;
    pub fn out_debug(text: &str);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! jsprintln {
    ($($arg:tt)*) => {{
        out_debug(&format!($($arg)*).to_string());
    }};
}

#[macro_export]
macro_rules! logprintln {
    ($($arg:tt)*) => {{
        log(&format!($($arg)*).to_string());
    }};
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
