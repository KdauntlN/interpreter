use std::fs;
use clap::Parser;
use display_tree::{format_tree, CharSet, DisplayTree, Style, StyleBuilder};

#[derive(Parser)]
struct Cli {
    filepath: String,
    #[arg(long)]
    dump_tokens: bool,
    #[arg(long)]
    dump_ast: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    IntLiteral(i64),
    Plus,
    Times,
    Divide,
    Subtract,
    LParen,
    RParen,
    Eof
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Token::IntLiteral(n) => write!(f, "{n}"),
            &Token::Plus => write!(f, "+"),
            &Token::Eof => write!(f, "EOF"),
            &Token::Times => write!(f, "*"),
            &Token::Divide => write!(f, "/"),
            &Token::Subtract => write!(f, "-"),
            &Token::LParen => write!(f, "("),
            &Token::RParen => write!(f, ")"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mult,
    Divide,
    Subtract,
    LParen,
    RParen,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Mult => write!(f, "*"),
            Op::Divide => write!(f, "/"),
            Op::Subtract => write!(f, "-"),
            Op::LParen => write!(f, "("),
            Op::RParen => write!(f, ")"),
        }
    }
}

enum Statement {
    Print(Expr)
}

#[derive(Debug, DisplayTree)]
enum Expr {
    IntLiteral(i64),
    BinaryOp { 
        #[node_label]
        op: Op,
        #[tree]
        left: Box<Expr>,
        #[tree]
        right: Box<Expr>
    }
}

struct Lexer {
    tokens: Vec<Token>
}

impl Lexer {
    fn tokenize(content: String) -> Lexer {
        let mut chars = content.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                let int = build_int(&mut chars, c);
                tokens.push(Token::IntLiteral(int));
            }

            if c == '+' {
                tokens.push(Token::Plus);
                continue;
            }

            if c == '*' {
                tokens.push(Token::Times);
                continue;
            }

            if c == '/' {
                tokens.push(Token::Divide);
                continue;
            }

            if c == '-' {
                tokens.push(Token::Subtract);
                continue;
            }

            if c == '(' {
                tokens.push(Token::LParen);
                continue;
            }

            if c == ')' {
                tokens.push(Token::RParen);
                continue;
            }
        }

        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

fn infix_binding_power(op: Op) -> (f32, f32) {
    match op {
        Op::Add => (1.0, 1.1),
        Op::Subtract => (1.0, 1.1),
        Op::Mult => (2.0, 2.1),
        Op::Divide => (2.0, 2.1),
        _ => panic!("Invalid operator: {op}"),
    }
}

fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expr {
    let mut left = match lexer.next() {
        Token::IntLiteral(i) => Expr::IntLiteral(i),
        Token::LParen => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next(), Token::RParen);
            lhs
        },
        t => panic!("Unexpected token: \"{t}\", expected atomic integer literal"),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::RParen => break,
            Token::Plus => Op::Add,
            Token::Subtract => Op::Subtract,
            Token::Times => Op::Mult,
            Token::Divide => Op::Divide,
            t => {
                println!("Current AST state: {left:#?}\nRemaining tokens: {:#?}", lexer.tokens);
                println!("Current token: {:#?}", lexer.peek());
                panic!("Unexpected token: \"{t}\", expected infix operator");
            },
        };

        let (l_bp, r_bp) = infix_binding_power(op);

        if l_bp < min_bp {
            break;
        }

        lexer.next();

        let right = parse_expression(lexer, r_bp);
        left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
    };

    left
}

fn build_int(chars: &mut std::iter::Peekable<impl Iterator<Item = char>>, first_digit: char) -> i64 {
    let mut int = String::from(first_digit);
    
    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            int.push(*c);
            chars.next();
        } else {
            println!("{c}");
            break;
        }
    }

    int.parse::<i64>().unwrap()
}

fn main() {
    let cli = Cli::parse();
    let content = fs::read_to_string(cli.filepath).unwrap();

    let mut lexer = Lexer::tokenize(content);

    if cli.dump_tokens {
        println!("Tokens: {:#?}", lexer.tokens);
    }

    if cli.dump_ast {
        let ast = parse_expression(&mut lexer, 0.0);
        let tree = format_tree!(ast, Style::default().indentation(1).char_set(CharSet::DOUBLE_LINE));
        println!("{tree}");
    }
}