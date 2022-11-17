use std::fmt::format;

use super::*; 

fn is_whitespace(c: char) -> bool {
    if c == ' ' || c == '\r' || c == '\n'|| c == '\t' {
        return true;
    }
    false
}

fn enough_space(src: &Vec<char>, idx: usize, space: usize, err: &str) {
    if src.len()-1 >= idx+space { return; }
    out_err(format!("ERROR: Invalid Syntax, {}", err).to_string().as_str());
}

fn make_word(src: &str, mut idx: usize) -> String {
    let mut ret = String::new();
    let indexable_src: Vec<char> = src.chars().collect();
    while is_whitespace(indexable_src[idx]) && idx != src.len()-1 { idx += 1; }
    while !is_whitespace(indexable_src[idx]) && idx != src.len()-1 {
        ret += indexable_src[idx].to_string().as_str();
        idx += 1;
    }
    ret
}


pub fn tokenise(src: &str) -> Vec<Token> {
    let mut toks: Vec<Token> = Vec::new();

    let mut i = 0;
    let mut buf: String = String::new();
    let mut is_str = false;
    let indexable_src: Vec<char> = src.chars().collect();

    while i < indexable_src.len()-1 {
        
        if indexable_src[i] == '\"' {
            is_str = !is_str;
            if !is_str { toks.push(Token::String(buf.to_string())); }
            enough_space(&indexable_src, i, 1, "EOF Before string ends.");
            i += 1+1;
            continue;
        }
        if is_str { buf += indexable_src[i].to_string().as_str(); i += 1; continue; }

        if indexable_src[i] == '\'' {
            enough_space(&indexable_src, i, 2, "EOF Before char ends");
            if indexable_src[i+2] != '\'' {
                out_err("ERROR: Char does not have ending");
                return vec![];
            }
            toks.push(Token::Char(indexable_src[i+1]));
            i += 2+1;
            continue;
        } else if indexable_src[i] == '.' {
            let mut j = 0;
            let mut val = String::new();
            while indexable_src[i+j] != ' ' && indexable_src[i+j] != '\r' && indexable_src[i+j] != '\t' && indexable_src[i+j] != '\n' && i <= indexable_src.len()-1 {
                val.push(indexable_src[i+j]);
                j += 1;
            }
            toks.push(Token::Label(val));
            i += j+1;
            continue;
        } else if indexable_src[i] == 'r' || indexable_src[i] == '$' {
            enough_space(&indexable_src, i, 1, "EOF Before address in register");
            let mut j = 0;
            let mut val = 0;
            while indexable_src[i+j].is_numeric() && indexable_src.len()-1 < i+j {
                val = val * 10 + indexable_src[i+j].to_string().parse::<i32>().unwrap();
                j += 1;
            }
            toks.push(Token::Register(val));
            i += j;
            continue;
        }
        

        // INSTRUCTIONS
        let word = make_word(src, i).to_lowercase();
        if word == "imm" || word == "rsh" || word == "lod" || word == "str" || word == "bge" 
            || word == "nor" || word == "jmp" { toks.push(Token::Instruction(word)); } 
        
        
        i += 1;
        
        while is_whitespace(indexable_src[i]) && i != indexable_src.len()-1 { i += 1; }
    }
    toks
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Label(String),
    Instruction(String),
    Minreg(i32),
    Minheap(i32),
    Minstack(i32),
    Bits(i32),
    Immediate(i32),
    DW,
    OpenBracket,
    CloseBracket,
    Register(i32),
    String(String),
    Char(char)
}