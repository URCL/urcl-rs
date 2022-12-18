use super::lexer::{UToken};

#[allow(dead_code)]
pub struct ErrorContext<'a> {
    errors: Vec<Error<'a>>
}
// REEEEEEEEEE LIFETIMES
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

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for error in &self.errors {
            output += error.kind.message();
            output += ": ";
            output += error.span;
            output.push('\n');
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
    NotEnoughOpperands,
    InvalidOpperandType,
    UnknownPort,
    DWNoEnding,
    EOFBeforeEndOfString,
    EOFBeforeEndOfChar,
    StackOverflow,
    StackUnderflow,
}

impl ErrorKind {
    pub fn message(&self) -> &str {
        match self {
            ErrorKind::UnknownPort => "Unknown port",
            ErrorKind::NotEnoughOpperands => "Not enough operands",
            ErrorKind::InvalidOpperandType => "Invalid operand type",
            ErrorKind::DWNoEnding => "todo!()",
            ErrorKind::EOFBeforeEndOfString => "End of file before the end of a string",
            ErrorKind::EOFBeforeEndOfChar => "End of file before the end of a char",
            ErrorKind::StackOverflow => "Stack overflow",
            ErrorKind::StackUnderflow => "Stack underflow",
        }
    }
}