use std::fs;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    filepath: String,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    IntLiteral(i64),
    Eof
}

enum Statement {
    Print(Expr)
}

enum Expr {
    IntLiteral(i64)
}

struct StmtParser {
    tokens: Vec<Token>,
    pos: usize
}

impl StmtParser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        token
    }

    fn check(&self, expected: &Token) -> bool {
        self.peek() == expected
    }

    fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
    }
}

fn tokenize(content: String) -> Vec<Token> {
    let mut chars = content.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            let int = build_int(&mut chars, c);
            tokens.push(Token::IntLiteral(int));
        }
    }

    tokens.push(Token::Eof);
    tokens
}

fn build_int(chars: &mut std::iter::Peekable<impl Iterator<Item = char>>, first_digit: char) -> i64 {
    let mut int = String::from(first_digit);
    
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            int.push(c);
        } else {
            break;
        }
    }

    int.parse::<i64>().unwrap()
}

fn main() {
    let cli = Cli::parse();
    let content = fs::read_to_string(cli.filepath).unwrap();

    let tokens = tokenize(content);
    println!("{tokens:#?}");
}