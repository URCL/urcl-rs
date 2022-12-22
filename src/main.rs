mod emulator;

use std::time::Instant;

fn main() {
    let mut emu = emulator::emulator::emulate("OUT 10 'h'\nHLT").unwrap();
    println!("{:?}", emu.run());
}

pub fn clear_text() {

}
pub fn in_text() -> String {
    "".to_owned()
}
pub fn out_text(text: &str) {
    println!("{}", text);
}

pub fn out_err(text: &str) {
    eprintln!("{}", text);
}

pub fn out_span(text: &str, class_name: &str) {
    println!(">{}", text);
}
pub fn clear_span() {

}

pub fn now() -> f64 {
    Instant::now().elapsed().as_secs_f64() * 1000.
}
pub fn out_debug(text: &str) {
    println!("{}", text);
}

pub fn out_screen(width: usize, height: usize, pixels: &[u32]) {
    println!("screen: {} {}", width, height);
}
pub fn clear_screen() {

}

fn log(s: &str) {
    println!("{}", s);
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