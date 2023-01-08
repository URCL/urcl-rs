#![cfg(not(target_family = "wasm"))]
mod emulator;

#[cfg(feature = "bot")]
mod discord_bot;

#[cfg(feature = "password")]
use base64;

#[allow(unused_imports)]
use toml::*;

use serde::*;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct SecretTOMLConfig {
    bot_key: String
}

fn main() {
    srand(now() as u64);
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
    
    #[cfg(feature = "bot")] #[allow(unused_must_use)] {
        use std::process::exit;
        let apikey = match std::fs::read("Secret.toml") {
            Ok(content) => {
                if (*content.iter().next().unwrap() as char).is_ascii_alphanumeric() {
                    let toml_c: SecretTOMLConfig = toml::from_str(std::str::from_utf8(&content).unwrap()).unwrap();
                    toml_c.bot_key.to_string()
                } else {
                    #[cfg(feature = "password")] {
                        let content = &content[1..content.len()];
                        let key = rpassword::prompt_password("Enter password: ").unwrap();
                        let offset = (key.chars().next().unwrap() as u16 % 3) + 1;
                        let decrypted = xor_encrypt(content.to_vec(), key.trim_end().as_bytes().to_vec(), 0);
                        let decrypted = std::str::from_utf8(&decrypted).unwrap();
                        let toml_c: SecretTOMLConfig = toml::from_str(&decrypted).expect("Password incorrect");
                        let bot_key = String::from_utf8(xor_encrypt(base64::decode(toml_c.bot_key).expect("Password incorrect"),
                            key.trim_end().as_bytes().to_vec(), offset
                        )).expect("Password incorrect");

                        bot_key
                    }
                    #[cfg(not(feature = "password"))] {
                        println!("Can't use passwords without password enabled!");
                        exit(1)
                    }
                }
            },
            _ => {
                let args: Vec<String> = std::env::args().collect();
                if args.len() <= 1 {
                    println!("\x1b[1;31mError: Not enough arguments.\x1b[0;0m");
                    return;
                }
                let fcontent: Vec<u8>;
                #[cfg(feature = "password")] {
                    let key = rpassword::prompt_password("Enter password (Enter nothing for not encrypting): ").unwrap();
                    let key = key.trim_end();

                    if key == "" {
                        fcontent = toml::to_string(&SecretTOMLConfig {bot_key: args[1].clone()}).unwrap().as_bytes().to_vec()
                    } else if key.len() < 8 {
                        println!("Password is too short! Password must be at lease 8 characters long!");
                        exit(1);
                    } else {
                        let offset = (key.chars().next().unwrap() as u16 % 3) + 1;
                        let bot_key_encrypted = xor_encrypt(args[1].clone().as_bytes().to_vec(), key.as_bytes().to_vec(), offset);
                        let mut a = xor_encrypt(toml::to_string(
                            &SecretTOMLConfig {
                                bot_key: base64::encode(bot_key_encrypted)
                            }
                        ).unwrap().as_bytes().to_vec(), key.as_bytes().to_vec(), 0);
                        a.insert(0, 0);
                        fcontent = a;
                    }
                }
                #[cfg(not(feature = "password"))] {
                    fcontent = toml::to_string(&SecretTOMLConfig {bot_key: args[1].clone()}).unwrap().as_bytes().to_vec();
                }

                match std::fs::write("Secret.toml", fcontent) {
                    Ok(_) => println!("\x1b[1;36mNote: URCL-rs sucessfully automatically added the Secret.toml file that stores your bot API key. DO NOT SHARE this file to other people\x1b[0;0m"),
                    _ => (),
                };
                args[1].clone()
            }
        };

        if let Err(err) = discord_bot::init_bot(&apikey) {
            println!("\x1b[1;31mError: Bot exited with error {err}.\x1b[0;0m");
        }
    }
}

#[cfg(feature = "password")]
fn xor_encrypt(s: Vec<u8>, k: Vec<u8>, offset: u16) -> Vec<u8> {
    let mut b = k.iter().cycle();
    for _ in 0..offset {b.next();}
    s.into_iter().map(|x| x ^ (b.next().unwrap() + (b.next().unwrap() << offset as u8) + (b.next().unwrap() >> (b.next().unwrap() % (offset as u8 + 1))))).collect()
}

pub fn clear_text() {

}

pub fn in_text() -> String {
    "".to_owned()
}

pub fn out_text(text: &str) {
    println!("{}", text);
}


static mut RAND_SEED: u64 = 0;

pub fn rand() -> u64 {
    unsafe {
        let mut x = RAND_SEED;
        if x == 0 {x = 1;}
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

#[allow(dead_code)]
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
