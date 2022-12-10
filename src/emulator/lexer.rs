use super::*;

use parse_int::parse;

fn is_whitespace(c: char) -> bool {
    if c == ' ' || c == '\r' || c == '\n'|| c == '\t' {
        return true;
    }
    false
}

fn enough_space(src: &Vec<char>, idx: usize, space: usize, err: &str) -> bool {
    if src.len() < idx+space { 
        out_html(format!("ERROR: Invalid Syntax, {}", err).as_str());
        return false;
    }
    true
}

fn make_word(src: &str, idx: &mut usize) -> String {
    let mut ret = String::new();
    let indexable_src: Vec<char> = src.chars().collect();
    while *idx < src.len() && is_whitespace(indexable_src[*idx])  { *idx += 1; }
    while *idx < src.len() && !is_whitespace(indexable_src[*idx]) {
        ret += &indexable_src[*idx].to_string();
        *idx += 1;
    }
    ret
}


pub fn tokenise(src: &str) -> Vec<Token> {
    let mut toks: Vec<Token> = Vec::new();
    
    let mut i = 0;
    let mut buf: String = String::new();
    let mut is_str = false;
    let mut indexable_src_tmp: Vec<char> = src.to_lowercase().chars().collect();
    indexable_src_tmp.push(' '); // whitespace to fix annoying bug
    let indexable_src = indexable_src_tmp;
    
    while i < indexable_src.len() {
        
        if indexable_src[i] == '\"' {
            is_str = !is_str;
            if !is_str { toks.push(Token::String(buf.to_string())); }
            if !enough_space(&indexable_src, i, 1, "EOF Before string ends.") { return vec![]; }
            i += 1+1;
            continue;
        }
        if is_str { buf += &indexable_src[i].to_string(); i += 1; continue; }
        
        if indexable_src[i] == '\'' {
            if !enough_space(&indexable_src, i, 1, "EOF Before string ends.") { return vec![]; }
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
            while i < indexable_src.len() && indexable_src[i+j] != ' ' && indexable_src[i+j] != '\r' && indexable_src[i+j] != '\t' && indexable_src[i+j] != '\n' {
                val.push(indexable_src[i+j]);
                j += 1;
            }
            toks.push(Token::Label(val));
            i += j+1;
            continue;
        } else if indexable_src[i] == 'r' || indexable_src[i] == '$' {
            if !enough_space(&indexable_src, i, 1, "EOF Before register ends.") { return vec![]; }
            let mut val = String::new();
            let mut j: usize = 1;
            while indexable_src.len() > i+j && (indexable_src[i+j].is_ascii_digit() || indexable_src[i+2] == 'x' || indexable_src[i+2] == 'b' || indexable_src[i+2] == 'o') {
                val += &indexable_src[i+j].to_string();
                j += 1;
            }
            
            if val.len() == 0 {
                out_err("ERROR: Invalid Register");
                return vec![];
            }
            toks.push(Token::Register(parse::<i32>(&val).unwrap()));
            i += j;
            continue;
        }
        

        
        // IMMEDIATES
        
        else if indexable_src[i].is_ascii_digit() { // this bit is shitting itself
            let mut val = String::new();
            let mut j: usize = 0;

            while indexable_src.len() > i+j && (indexable_src[i+j].is_ascii_digit() || indexable_src[i+1] == 'x' || indexable_src[i+1] == 'b' || indexable_src[i+1] == 'o') {
                val += &indexable_src[i+j].to_string();
                j = j+1;
            }
            toks.push(Token::Immediate(parse::<i32>(&val.to_lowercase()).unwrap()));
            i += j;
            continue;
        }
        

        // INSTRUCTIONS
        let word = make_word(src, &mut i).to_lowercase();
        if word == "imm" || word == "rsh" || word == "lod" || word == "str" || word == "bge" 
            || word == "nor" || word == "jmp" { toks.push(Token::Instruction(word)); } 
        
        i += 1;
        
        while  i < indexable_src.len() && is_whitespace(indexable_src[i]) { i += 1; }
    }
    toks
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
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