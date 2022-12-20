use std::fmt::{Debug, Display};

use super::{lexer::{UToken}, ast::AstOp};

#[allow(dead_code)]
pub struct ErrorContext<'a> {
    errors: Vec<Error<'a>>
}
// REEEEEEEEEE LIFETIMES
impl <'a> ErrorContext<'a> {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn error(&mut self, token: &UToken<'a>, kind: ErrorKind<'a>) {
        self.errors.push(Error {kind, span: token.str});
    }
    pub fn has_error(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn to_string(&self, src: &str) -> String {
        let mut output = String::new();
        for error in &self.errors {
            let (line, col) = line(src, error.span);
            output += &format!("{}\n{}{}----- {}\n",
                line,
                " ".repeat(col),
                "^".repeat(error.span.chars().count().max(1)),
                error.kind
            );
        }
        output
    }
}

#[allow(dead_code)]
pub struct Error<'a> {
    kind: ErrorKind<'a>,
    span: &'a str, // start and end of code that caused the error
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorKind<'a> {
    NotEnoughOperands,
    ToManyOperands,
    InvalidOperandType{expected: &'a str, actual: AstOp},
    InvalidOperand,
    UndefinedLabel,
    UnknownPort,
    DWNoEnding,
    EOFBeforeEndOfString,
    EOFBeforeEndOfChar,
    StackOverflow,
    StackUnderflow,
}
impl <'a> Display for ErrorKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughOperands => write!(f, "Not enough operands"),
            Self::ToManyOperands => write!(f, "Too many operands"),
            Self::InvalidOperandType { expected, actual } => write!(f, "Expected operand {} but got {:?}", expected, actual),
            Self::UnknownPort => write!(f, "Unknown port"),
            Self::DWNoEnding => write!(f, "Missing ']'"),
            Self::EOFBeforeEndOfString => write!(f, "Missing '\"'"),
            Self::EOFBeforeEndOfChar => write!(f, "Missing '''"),
            Self::StackOverflow => write!(f, "Stack overflow"),
            Self::StackUnderflow => write!(f, "Stack underflow"),
            Self::InvalidOperand => write!(f, "Invalid operand"),
            Self::UndefinedLabel => write!(f, "Undefined label")
        }
    }
}
const LF_B: u8 = '\n' as u8;

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