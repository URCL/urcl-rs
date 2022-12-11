use std::{iter::Peekable, str::CharIndices};

pub type UToken<'a> = Token<'a, Kind>;

#[derive(Debug, Clone)]
pub enum Kind {
    Unknown, White, Error, Name, Macro,
    Int(u64), Memory, Port, Reg, Label,
    Eq, GE, LE,
    LSquare, RSquare, String, Char, Text, Escape(char),
}

pub fn lex(src: &str) -> Vec<Token<Kind>>{
    use Kind::*;
    let mut s = Scanner::<Kind>::new(src);
    
    while let Some(c) = s.next() {
        match c {
            '[' => {s.create(LSquare)},
            ']'=> {s.create(RSquare);}
            ' ' | '\x09'..='\x0d' => {s._while(char::is_whitespace); s.create(White);},
            '-' | '+' | '0'..='9' => {
                s._while(|c|c.is_ascii_digit());
                let value = s.str().parse().unwrap();
                s.create(Int(value))
            },
            '#' | 'm' | 'M' => {s._while(char::is_alphanumeric); s.create(Memory)},
            '$' | 'r' | 'R' => {s._while(char::is_alphanumeric); s.create(Reg)},
            '@' => {s._while(char::is_alphanumeric); s.create(Macro)},
            '%' => {s._while(char::is_alphanumeric); s.create(Port)},
            'a'..='z' | 'A'..='Z' => {s._while(char::is_alphanumeric); s.create(Name)},
            '.' => {s._while(char::is_alphanumeric); s.create(Label)},
            '\'' => {
                s.create(Char);
                if let Some(c) = s.next() {
                    if c == '\\' {
                        token_escape(&mut s);
                    } else {
                        s.create(Text);
                        if let Some(c) = s.next() {
                            if c == '\'' {
                                s.create(Char);
                            } else {
                                s.create(Error)
                            }
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
                        _ => {s.next(); has_text = true;}
                    }
                }
            },
            _ => {s.create(Unknown)}
        }
    }

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
    pub fn cssClass(&self) -> &'static str {
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
        while self.chars.next_if(|(_, c)| f(*c)).is_some() {}
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
