use super::lexer::{UToken};

#[allow(dead_code)]
pub struct ErrorContext<'a> {
    errors: Vec<Error<'a>>
}

impl <'a> ErrorContext<'a> {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn error(&mut self, token: &UToken<'a>, kind: ErrorKind) {
        self.errors.push(Error {kind, span: token.str});
    }
    pub fn has_error(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn to_string(&self, src: &str) -> String {
        let mut output = String::new();
        for error in &self.errors {
            let (line, col) = line(src, error.span);
            output += &format!("{line}\n{}{}----- {}: {}\n", &" ".repeat(col), &"^".repeat(error.span.chars().count().max(1)), error.kind.message(), error.span);
        }
        output
    }
}

#[allow(dead_code)]
pub struct Error<'a> {
    kind: ErrorKind,
    span: &'a str, // start and end of code that caused the error
}

#[allow(dead_code)]
pub enum ErrorKind {
    NotEnoughOperands,
    ToManyOperands,
    InvalidOperandType,
    UnknownPort,
    UnknownInstruction,
    DWNoEnding,
    EOFBeforeEndOfString,
    EOFBeforeEndOfChar,
    StackOverflow,
    StackUnderflow,
}

impl ErrorKind {
    pub fn message(&self) -> &str {
        match self {
            ErrorKind::NotEnoughOperands => "Not enough operands",
            ErrorKind::ToManyOperands => "To many operands",
            ErrorKind::InvalidOperandType => "Invalid operand type",
            ErrorKind::UnknownPort => "Unknown port",
            ErrorKind::UnknownInstruction => "Unknown instruction",
            ErrorKind::DWNoEnding => "todo!()",
            ErrorKind::EOFBeforeEndOfString => "End of file before the end of a string",
            ErrorKind::EOFBeforeEndOfChar => "End of file before the end of a char",
            ErrorKind::StackOverflow => "Stack overflow",
            ErrorKind::StackUnderflow => "Stack underflow",
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