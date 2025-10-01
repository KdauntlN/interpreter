use crate::lexer::{Lexer, Token};

use clap::Parser;
use display_tree::DisplayTree;

pub mod lexer;

#[derive(Parser)]
pub struct Cli {
    pub filepath: String,
    #[arg(long)]
    pub dump_tokens: bool,
    #[arg(long)]
    pub dump_ast: bool,
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
pub enum Op {
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

pub enum Statement {
    Print(Expr)
}

#[derive(Debug, DisplayTree)]
pub enum Expr {
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

fn infix_binding_power(op: Op) -> (f32, f32) {
    match op {
        Op::Add => (1.0, 1.1),
        Op::Subtract => (1.0, 1.1),
        Op::Mult => (2.0, 2.1),
        Op::Divide => (2.0, 2.1),
        _ => panic!("Invalid operator: {op}"),
    }
}

pub fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expr {
    let mut left = match lexer.next() {
        Token::IntLiteral(i) => Expr::IntLiteral(i),
        Token::LParen => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next(), Token::RParen);
            lhs
        },
        t => panic!("Got \"{t}\', expected int"),
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