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
    
    pub fn out_span(text: &str, class_name: &str);
    pub fn clear_span();
    pub fn out_linenumber(text: &str);
    
    pub fn now() -> f64;
    pub fn out_debug(text: &str);

    pub fn out_screen(width: usize, height: usize, pixels: &[u32]);
    pub fn clear_screen();
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! jsprintln {
    ($($arg:tt)*) => {{
        // out_text wont add new line
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

pub fn out_err(out: &mut String, error: &emulator::errorcontext::Error, lineno: &String, line: &str, col: usize) {
    use std::fmt::Write;
    use crate::emulator::errorcontext::*;
    writeln!(out, "<span class=\"{}\">{}: {}</span>",
        format!("{}", error.level).to_lowercase(), error.level, error.kind
    ).unwrap();
    writeln!(out, "{}| {}", 
        lineno, html_escape::encode_text(&line.split_at(get_indent_level(line)).1.replace("\t", " "))
    ).unwrap();
    writeln!(out, "{}| {}{}",
        " ".repeat(str_width(lineno)),
        &" ".repeat(col - get_indent_level(line)),
        &"^".repeat(str_width(error.span).max(1))
    ).unwrap();
}

pub fn out_emu_err(out: &mut String, error: &emulator::emulator::EmulatorErrorKind, lineno: &String, line: &str) {
    use std::fmt::Write;
    use crate::emulator::errorcontext::*;
    writeln!(out, "<span class=\"error\">Error: {}</span>", error).unwrap();
    writeln!(out, "{}| {}", 
        lineno, html_escape::encode_text(&line.split_at(get_indent_level(line)).1.replace("\t", " "))
    ).unwrap();
}

static mut RAND_SEED: u64 = 0;

pub fn rand() -> u64 {
    unsafe {
        let mut x = RAND_SEED;
        if x == 0 {x = now() as u64;}
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        RAND_SEED = x;
        x
    }
}

pub fn srand(seed: u64) {
    unsafe {
        RAND_SEED = seed;
    }
}
