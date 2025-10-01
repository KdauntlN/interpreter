use std::{
    iter::Peekable,
    str::Chars,
    rc::Rc,
    fmt::{
        self,
        Display,
        Formatter,
    }
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    IntLiteral(i64),
    Identifier(Rc<str>),
    Plus,
    Times,
    Divide,
    Subtract,
    LParen,
    RParen,
    KwPrint,
    Semicolon,
    Eof
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::IntLiteral(n) => write!(f, "{n}"),
            Token::Identifier(x) => write!(f, "{x}"),
            Token::Plus => write!(f, "+"),
            Token::Eof => write!(f, "EOF"),
            Token::Times => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Subtract => write!(f, "-"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::KwPrint => write!(f, "print"),
            Token::Semicolon => write!(f, ";")
        }
    }
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { chars: source.chars().peekable() }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.chars.next() {
            if c.is_ascii_digit() {
                let int = self.build_int(c);
                tokens.push(Token::IntLiteral(int));
            }

            if c.is_ascii_alphabetic() || c == '_' {
                let ident = self.build_ident(c);
                match find_keyword(&ident) {
                    Some(token) => tokens.push(token),
                    None => tokens.push(Token::Identifier(ident)),
                }
            }

            tokens.push(match c {
                '+' => Token::Plus,
                '*' => Token::Times,
                '/' => Token::Divide,
                '-' => Token::Subtract,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ';' => Token::Semicolon,
                _ => continue,
            });
            continue;

        }

        tokens
    }

    fn build_int(&mut self, first_digit: char) -> i64 {
        let mut int = String::from(first_digit);
        
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_digit() {
                int.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        int.parse::<i64>().unwrap()
    }

    fn build_ident(&mut self, first_letter: char) -> Rc<str> {
        let mut identifier = String::from(first_letter);

        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() || *c == '_' {
                identifier.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        Rc::from(identifier)
    }
}

fn find_keyword(ident: &str) -> Option<Token> {
    match ident {
        "print" => Some(Token::KwPrint),
        _ => None
    }
}