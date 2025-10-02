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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub line_number: i32,
    pub col: i32,
    pub length: i32,
}

impl Token {
    pub fn new(kind: TokenKind, line_number: i32, col: i32, length: i32) -> Self {
        Self {
            kind,
            line_number,
            col,
            length
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::IntLiteral(n) => write!(f, "{n}"),
            TokenKind::Identifier(x) => write!(f, "{x}"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Times => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::Subtract => write!(f, "-"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::KwPrint => write!(f, "print"),
            TokenKind::Semicolon => write!(f, ";")
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

    pub fn tokenize(&mut self) -> (Vec<Token>, i32, i32) {
        let mut tokens: Vec<Token> = Vec::new();
        let (mut line_number, mut col) = (1, 1);

        while let Some(c) = self.chars.next() {
            if c.is_ascii_digit() {
                let (int, length) = self.build_int(c);
                col += length;
                tokens.push(Token::new(TokenKind::IntLiteral(int), line_number, col, length));
                continue;
            }

            if c.is_ascii_alphabetic() || c == '_' {
                let ident = self.build_ident(c);
                let len = ident.len() as i32;
                col += len;
                match find_keyword(&ident) {
                    Some(keyword) => tokens.push(Token::new(keyword, line_number, col, len)),
                    None => tokens.push(Token::new(TokenKind::Identifier(ident.clone()), line_number, col, len)),
                }
                continue;
            }

            if c == '\n' {
                line_number += 1;
                col = 0;
            }

            tokens.push(match c {
                '+' => Token::new(TokenKind::Plus, line_number, col, 1),
                '*' => Token::new(TokenKind::Times, line_number, col, 1),
                '/' => Token::new(TokenKind::Divide, line_number, col, 1),
                '-' => Token::new(TokenKind::Subtract, line_number, col, 1),
                '(' => Token::new(TokenKind::LParen, line_number, col, 1),
                ')' => Token::new(TokenKind::RParen, line_number, col, 1),
                ';' => Token::new(TokenKind::Semicolon, line_number, col, 1),
                _ => {
                    col += 1;
                    continue;
                },
            });

            continue;

        }

        (tokens, line_number, col)
    }

    fn build_int(&mut self, first_digit: char) -> (i64, i32) {
        let mut int = String::from(first_digit);
        
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_digit() {
                int.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        let length = int.len() as i32;
        let int = int.parse::<i64>().unwrap();

        (int, length)
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

fn find_keyword(ident: &str) -> Option<TokenKind> {
    match ident {
        "print" => Some(TokenKind::KwPrint),
        _ => None
    }
}