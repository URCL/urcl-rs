#![cfg(not(target_family = "wasm"))]
mod emulator;

use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("\x1b[1;31mError: Not enough arguments.\x1b[0;0m");
        return;
    }
    let fname = &args[1];
    let src = std::fs::read_to_string(fname);
    match &src {Err(err) => {
        println!("\x1b[1;31mError: Cannot read file {} (Returns error \"{}\")\x1b[0;0m", fname, err);
        return;
    }, _ => ()}
    let emu = emulator::emulator::emulate(src.unwrap());
    match emu {
        None => {
            println!("\x1b[1;31mError: Compilation failed\x1b[0;0m");
            return;
        }
        _ => (),
    } 
    println!("{:?}", emu.unwrap().run());
}

pub fn clear_text() {

}

pub fn in_text() -> String {
    "".to_owned()
}

pub fn out_text(text: &str) {
    println!("{}", text);
}

pub fn out_err(out: &mut String, error: &emulator::errorcontext::Error, lineno: &String, line: &str, col: usize) {
    use std::fmt::Write;
    use crate::emulator::errorcontext::*;
    writeln!(out, "\x1b[1;{}m{}: {}\x1b[0;0m",
        match error.level {
            ErrorLevel::Info    => 96,
            ErrorLevel::Warning => 93,
            ErrorLevel::Error   => 91,
        }, error.level, error.kind
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

pub fn out_span(text: &str, _class_name: &str) {
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

pub fn out_screen(width: usize, height: usize, _pixels: &[u32]) {
    println!("screen: {} {}", width, height);
}

pub fn clear_screen() {

}

pub fn out_linenumber(_: &str) {}

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
