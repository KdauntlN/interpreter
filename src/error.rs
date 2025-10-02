use crate::lexing::{Token, TokenKind};
use std::rc::Rc;

#[derive(Debug)]
pub struct Error {
    filename: Rc<str>,
    kind: ErrorKind,
    pub line: i32,
    col: i32
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedToken{
        provided: TokenKind,
        token_length: i32,
    },
    ZeroDivision,
}

impl Error {
    pub fn wrong_token(filename: Rc<str>, provided: Token) -> Self {
        Self {
            filename,
            kind: ErrorKind::UnexpectedToken {
                provided: provided.kind,
                token_length: provided.length,
            },
            line: provided.line_number,
            col: provided.col
        }
    }

    pub fn format_err(&self, line: &str) -> String {
        let mut message: Vec<String> = Vec::new();

        match &self.kind {
            ErrorKind::UnexpectedToken { provided, token_length } => {
                let width = self.line.to_string().len();
                message.push(format!("unexpected token '{}' in '{}' at line {}, column {}", provided, self.filename, self.line, self.col));
                message.push(format!("{:>width$} |", " ", width = width));
                message.push(format!("{} | {}", self.line, line));
                message.push(format!("{:>width$} | {}", " ", "^".repeat(*token_length as usize), width = width));

                message.join("\n")
            },
            ErrorKind::ZeroDivision => String::from("Can't divide by zero lmao"),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::UnexpectedToken { provided, token_length} => {
                write!(f, "Unexpected token {provided} of length {token_length} found.")
            }
            ErrorKind::ZeroDivision => write!(f, "Cannot divide by zero"),
        }
    }
}

impl std::error::Error for Error {}