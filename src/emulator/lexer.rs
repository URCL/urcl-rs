use super::*; 
/*
fn make_word(src: &str, idx: usize) -> &str {
    
}*/


pub fn tokenise(src: &str) -> Vec<Token> {

    let mut toks: Vec<Token> = Vec::new();

    let mut i = 0;
    let mut buf: String = String::new();
    let mut is_str = false;
    let indexable_src: Vec<char> = src.chars().collect();

    while i < src.len() {
        if indexable_src[i] == '\"' {
            is_str = !is_str;
            if !is_str { toks.push(Token::String(buf.to_string())); }
            i += 1;
            continue;
        }
        if is_str { buf += indexable_src[i].to_string().as_str(); i += 1; continue; }

        if indexable_src[i] == '\'' {
            if indexable_src[i+2] != '\'' {
                out_err("ERROR: Char does not have ending");
                return vec![];
            }
            toks.push(Token::Char(indexable_src[i+1]));
            i += 2;
        } else if indexable_src[i] == '.' {
            let mut j = 0;
            let mut val = String::new();
            while indexable_src[i+j] != ' ' && indexable_src[i+j] != '\r' && indexable_src[i+j] != '\t' && indexable_src[i+j] != '\n' {
                val.push(indexable_src[i+j]);
                j += 1;
            }
            toks.push(Token::Label(val));
            i += j;
        }

        i += 1;
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