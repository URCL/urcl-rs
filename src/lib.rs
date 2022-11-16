use wasm_bindgen::prelude::*;
mod emulator;

#[wasm_bindgen(raw_module="../script.js")]
extern {
    pub fn out_text(text: &str);
    pub fn out_graphics(x: u64, y: u64, colour: u64);
    pub fn in_text() -> String;
    pub fn out_err(text: &str);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace=console)]
    fn _log(s: &str);
}

#[macro_export]
macro_rules! jsprintln {
    ($($arg:tt)*) => {{
        _log(&format!($($arg)*).to_string());
    }};
}