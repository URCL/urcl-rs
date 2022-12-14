use std::{iter::Peekable, str::CharIndices};
use super::*;

pub type UToken<'a> = Token<'a, Kind>;

#[derive(Debug, Clone)]
pub enum Kind {
    Unknown, Error, Comment,
    White, Name, Macro, 
    Int(i64), Memory, Port, Reg, Label, Relative(i64),
    Eq, GE, LE,
    LSquare, RSquare, String, Char, Text, Escape(char),
}

pub fn lex(src: &str) -> Vec<Token<Kind>>{
    use Kind::*;
    let mut s = Scanner::<Kind>::new(src);

    while let Some(c) = s.next() {
        match c {
            '[' => {s.create(LSquare)},
            ']' => {s.create(RSquare);}
            ' ' | '\x09'..='\x0d' => {s._while(char::is_whitespace); s.create(White);},
            '0' => {
                match s.peek().unwrap_or('0') {
                    '0'..='9' => {
                        s.next();
                        s._while(|c|c.is_ascii_digit());
                        let value = s.str().parse().unwrap_or(0);
                        s.create(Int(value));
                    },
                    'b' => {
                        s.next();
                        s._while(|c|c == '0' || c == '1');
                        if s.str().len() <= 2 { s.create(Error); continue; }
                        let value = i64::from_str_radix(s.str(), 2).unwrap_or(0);
                        s.create(Int(value));
                    },
                    'o' => {
                        s.next();
                        s._while(|c|c.is_ascii_digit() && c != '8' && c != '9');
                        if s.str().len() <= 2 { s.create(Error); continue; }
                        let value = i64::from_str_radix(s.str(), 8).unwrap_or(0);
                        s.create(Int(value));
                    },
                    'x' => {
                        s.next();
                        s._while(|c|c.is_ascii_hexdigit());
                        if s.str().len() <= 2 { s.create(Error); continue; }
                        let value = i64::from_str_radix(&s.str()[2..s.str().len()], 16).unwrap_or(0);
                        s.create(Int(value));
                    },
                    _ => {
                        s.create(Int(0));
                    }
                }
            },
            '-' | '+' | '1'..='9' => {
                s._while(|c|c.is_ascii_digit());
                let value = s.str().parse().unwrap_or(0);
                s.create(Int(value))
            },
            '~' => {
                s._while(|c|c.is_ascii_digit() || c == '-' || c == '+');
                let mut a = s.str().to_string(); a.remove(0);
                s.create(Relative(a.parse().unwrap_or(0)))
            },
            '#' | 'm' | 'M' => {
                if s.peek().unwrap_or(' ').is_ascii_digit() {
                    s._while(char::is_alphanumeric); s.create(Memory);
                } else {
                    s._while(char::is_alphanumeric); s.create(Name);
                }},
            '$' | 'r' | 'R' => {
                if s.peek().unwrap_or(' ').is_ascii_digit() {
                    s._while(char::is_alphanumeric); s.create(Reg);
                } else {
                    s._while(char::is_alphanumeric); s.create(Name);
                }},
            '@' => {s._while(char::is_alphanumeric); s.create(Macro)},
            '%' => {s._while(char::is_alphanumeric); s.create(Port)},
            'a'..='z' | 'A'..='Z' => {s._while(char::is_alphanumeric); s.create(Name)},
            '.' => {s._while(|c|c != ' ' && c != '\n' && c != '\t'); s.create(Label)},
            '/' => {if s._if(|c| c == '/') {
                s._while(|c| c != '\n');
                s.create(Comment);
            } else if s._if(|c| c == '*') {
                while s.next().map_or(false, |c| c != '*') || s.next().map_or(false, |c| c != '/'){}
                s.create(Comment);
            } else {
                s.create(Error);
            }},
            '\'' => {
                s.create(Char);
                if let Some(c) = s.next() {
                    if c == '\\' {
                        token_escape(&mut s);
                    } else {
                        s.create(Text);
                    }
                    if let Some(c) = s.next() {
                        if c == '\'' {
                            s.create(Char);
                        } else {
                            s.create(Error);
                        }
                    }
                }
            },
            '"' => {
                s.create(String);
                let mut has_text = false;
                while let Some(c) = s.peek() {
                    match c {
                        '\\' => {
                            if has_text {s.create(Text);}
                            s.next();
                            has_text = false;
                            token_escape(&mut s);
                        },
                        '"' => {
                            if has_text {s.create(Text);}
                            s.next();
                            s.create(String);
                            break;
                        },
                        '\n' => {
                            s.create(Error);
                            break;
                        },
                        _ => {
                            s.next(); has_text = true;
                        }
                    }
                }
            },
            _ => {s.create(Unknown)}
        }
    }
    s.create(Error);

    s.tokens()
}


fn token_escape<'a>(s: &mut Scanner<'a, Kind>) {
    use Kind::*;
    if let Some(c) = s.next() {
        match c {
            't' => s.create(Escape('\t')),
            'r' => s.create(Escape('\r')),
            'n' => s.create(Escape('\n')),
            '"' => s.create(Escape('\"')),
            '\'' => s.create(Escape('\'')),
            _ => s.create(Error),
        }
    } else {
        s.create(Error);
    }
}

impl Kind {
    pub fn css_class(&self) -> &'static str {
        match self {
            Kind::Unknown => "unknown",
            Kind::White => "white",
            Kind::Int(_) => "int",
            Kind::LSquare => "left-square",
            Kind::RSquare => "right-square",
            Kind::String => "string",
            Kind::Char => "char",
            Kind::Text => "text",
            Kind::Escape(_) => "escape",
            Kind::Error => "error",
            Kind::Memory => "memory",
            Kind::Port => "port",
            Kind::Reg => "reg",
            Kind::Name => "name",
            Kind::Macro => "macro",
            Kind::Eq => "comparison",
            Kind::GE => "comparison",
            Kind::LE => "comparison",
            Kind::Label => "label",
            Kind::Comment => "comment",
            Kind::Relative(_) => "relative",
        }
    }
}


#[derive(Debug, Clone)]
pub struct Token<'a, T> {
    pub kind: T,
    pub str: &'a str,
}

pub struct Scanner <'a, T> {
    src: &'a str,
    chars: Peekable<CharIndices<'a>>,
    start: usize,
    tokens: Vec<Token<'a, T>>,
}

impl <'a, T> Scanner<'a, T> {
    pub fn new(src: &'a str) -> Self {
        Self {src, chars: src.char_indices().peekable(), start: 0, tokens: Vec::new()}
    }
    #[inline]
    pub fn peek(&mut self) -> Option<char>{
        self.chars.peek().map(|(_, c)| {*c})
    }
    #[inline]
    pub fn _while<F: Fn(char) -> bool>(&mut self, f: F){
        while self._if(|c| f(c)) {}
    }
    #[inline]
    pub fn _if<F: Fn(char) -> bool>(&mut self, f: F) -> bool {
        self.chars.next_if(|(_, c)| f(*c)).is_some()
    }
    #[inline]
    pub fn create(&mut self, kind: T) {
        let start = self.start;
        let end = self.chars.peek().map(|(i, _)| *i).unwrap_or(self.src.len());
        self.start = end;

        let str = &self.src[start..end];
        self.tokens.push(Token { kind, str });
    }
    #[inline]
    pub fn str(&mut self) -> &'a str{
        let start = self.start;
        let end = self.chars.peek().map(|(i, _)| *i).unwrap_or(self.src.len());
        &self.src[start..end]
    }
    pub fn tokens(self) -> Vec<Token<'a, T>> {
        self.tokens
    }
}
impl <'a, T> Iterator for Scanner<'a, T> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|(_, c)| c)
    }
}
