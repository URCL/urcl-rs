use super::lexer::UToken;
use std::ops::Range;

#[allow(dead_code)]
pub struct ErrorContext {
    errors: Vec<Error>
}

impl ErrorContext {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
}

#[allow(dead_code)]
pub struct Error {
    kind: ErrorKind,
    span: Range<usize>, // start and end of code that caused the error
}

#[allow(dead_code)]
pub enum ErrorKind {
    NotEnoughOpperands,
    InvalidOpperandType,
    DWNoEnding,
    EOFBeforeEndOfString,
    EOFBeforeEndOfChar,
    StackOverflow,
    StackUnderflow,
}