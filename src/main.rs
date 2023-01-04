#![cfg(not(target_family = "wasm"))]
mod emulator;

#[cfg(feature = "bot")]
mod discord_bot;


fn main() {
    #[cfg(not(feature = "bot"))] {
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

    #[cfg(feature = "bot")] {
        let args: Vec<String> = std::env::args().collect();
        if args.len() <= 1 {
            println!("\x1b[1;31mError: Not enough arguments.\x1b[0;0m");
            return;
        }

        if let Err(err) = discord_bot::init_bot(&args[1]) {
            println!("\x1b[1;31mError: Bot exited with error {err}.\x1b[0;0m");
        }
    }
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
    #[cfg(not(feature = "bot"))] {
        writeln!(out, "\x1b[1;{}m{}: {}\x1b[0;0m",
            match error.level {
                ErrorLevel::Info    => 36,
                ErrorLevel::Warning => 33,
                ErrorLevel::Error   => 31,
            }, error.level, error.kind
        ).unwrap();
        writeln!(out, "\t{}| {}", 
            lineno, html_escape::encode_text(&line.split_at(get_indent_level(line)).1.replace("\t", " "))
        ).unwrap();
        writeln!(out, "\t{}| {}{}",
            " ".repeat(str_width(lineno)),
            &" ".repeat(col - get_indent_level(line)),
            &"^".repeat(str_width(error.span).max(1))
        ).unwrap();
    }
    #[cfg(feature = "bot")] {
        writeln!(out, "\x1b[1;{}m{}: {}\x1b[0;0m",
            match error.level {
                ErrorLevel::Info    => 36,
                ErrorLevel::Warning => 33,
                ErrorLevel::Error   => 31,
            }, error.level, error.kind
        ).unwrap();
        writeln!(out, "\t{}| {}", 
            lineno, html_escape::encode_text(&line.split_at(get_indent_level(line)).1.replace("\t", " "))
        ).unwrap();
        writeln!(out, "\t{}| {}{}",
            " ".repeat(str_width(lineno)),
            &" ".repeat(col - get_indent_level(line)),
            &"^".repeat(str_width(error.span).max(1))
        ).unwrap();
    }
}

pub fn out_emu_err(out: &mut String, error: &emulator::emulator::EmulatorErrorKind, lineno: &String, line: &str) {
    use std::fmt::Write;
    use crate::emulator::errorcontext::*;
    if !cfg!(feature = "bot") {
        writeln!(out, "\x1b[1;31mError: {}\x1b[0;0m", error).unwrap();
        writeln!(out, "\t{}| {}", 
            lineno, &line.split_at(get_indent_level(line)).1.replace("\t", " ")
        ).unwrap();
    } else {
        writeln!(out, "\x1b[1;31mError: {}\x1b[0;0m", error).unwrap();
        writeln!(out, "\t{}| {}", 
            lineno, &line.split_at(get_indent_level(line)).1.replace("\t", " ")
        ).unwrap();
    }
}


pub fn out_span(text: &str, _class_name: &str) {
    println!(">{}", text);
}

pub fn clear_span() {

}

pub fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64
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
