use std::{fmt::{Debug, Display}, collections::HashMap};

use strum_macros::Display;

use super::{lexer::{UToken}, ast::AstOp};

#[allow(dead_code)]
pub struct ErrorContext<'a> {
    errors: Vec<Error<'a>>,
    has_error: bool,
}

#[allow(dead_code)]
impl <'a> ErrorContext<'a> {
    pub fn new() -> Self {
        Self { errors: Vec::new(), has_error: false }
    }

    pub fn error(&mut self, token: &UToken<'a>, kind: ErrorKind<'a>) {
        self.errors.push(Error {kind, span: token.str, level: ErrorLevel::Error});
        self.has_error = true;
    }
    pub fn warn(&mut self, token: &UToken<'a>, kind: ErrorKind<'a>) {
        self.errors.push(Error {kind, span: token.str, level: ErrorLevel::Warning});
    }
    pub fn info(&mut self, token: &UToken<'a>, kind: ErrorKind<'a>) {
        self.errors.push(Error {kind, span: token.str, level: ErrorLevel::Info});
    }
    pub fn has_error(&self) -> bool {
        self.has_error
    }

    pub fn to_string(&self, src: &str) -> String {
        let mut linenos = HashMap::new();
        for (i, line) in src.lines().enumerate() {
            linenos.insert(line.as_ptr(), i+1);
        }

        let mut output = String::new();
        for error in &self.errors {
            let (line, col) = line(src, error.span);
            let lineno = linenos.get(&line.as_ptr()).map_or(0, |i|*i);
            let lineno = format!("{} ", lineno);

            crate::out_err(&mut output, error, &lineno, line, col);
        }
        output
    }
}



pub fn get_indent_level(src: &str) -> usize {
    let mut tabs = 0;
    for el in src.chars() {
        if el != '\t' && el != ' ' {break} else {tabs += 1}
    }
    tabs
}

#[derive(Debug, Display)]
pub enum ErrorLevel {
    Info, Warning, Error
}

#[allow(dead_code)]
pub struct Error<'a> {
    pub kind: ErrorKind<'a>,
    pub span: &'a str, // start and end of code that caused the error
    pub level: ErrorLevel
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorKind<'a> {
    UnexpectedMacro,
    NotEnoughOperands,
    ToManyOperands,
    InvalidOperandType{expected: &'a str, actual: AstOp},
    InvalidOperand,
    UndefinedLabel,
    UnknownPort,
    UnknownInstruction,
    DWNoEnding,
    EOFBeforeEndOfString,
    EOFBeforeEndOfChar,
    DuplicatedLabelName,
    YoMamma
}
impl <'a> Display for ErrorKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::UnexpectedMacro => write!(f, "Unexpected macro"),
            ErrorKind::NotEnoughOperands => write!(f, "Not enough operands"),
            ErrorKind::ToManyOperands => write!(f, "Too many operands"),
            ErrorKind::InvalidOperandType { expected, actual } => write!(f, "Expected operand {} but got {:?}", expected, actual),
            ErrorKind::UnknownPort => write!(f, "Unknown port"),
            ErrorKind::DWNoEnding => write!(f, "Missing ']'"),
            ErrorKind::EOFBeforeEndOfString => write!(f, "Missing '\"'"),
            ErrorKind::EOFBeforeEndOfChar => write!(f, "Missing '''"),
            ErrorKind::InvalidOperand => write!(f, "Invalid operand"),
            ErrorKind::UndefinedLabel => write!(f, "Undefined label"),
            ErrorKind::DuplicatedLabelName => write!(f, "Duplicated label name"),
            ErrorKind::UnknownInstruction => write!(f, "Unknown instruction"),
            ErrorKind::YoMamma => write!(f, "Token too large")
        }
    }
}
const LF_B: u8 = '\n' as u8;

pub fn str_width(src: &str) -> usize {
    src.chars().count()
}

fn line<'a>(src: &'a str, span: &'a str) -> (&'a str, usize) {
    let mut offset = span.as_ptr() as usize - src.as_ptr() as usize;
    if offset >= src.len() {
        offset = src.len();
    }
    drop(span);
    let mut start = offset;
    while start > 0 && src.as_bytes()[start - 1] != LF_B  {
        start -= 1;
    }
    let mut end = offset;
    while end < src.len() && src.as_bytes()[end] != LF_B {
        end += 1;
    }
    (&src[start..end], src[start..offset].chars().count())
}
