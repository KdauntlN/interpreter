use crate::{
    lexing::Token,
    TokenStream
};

use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Add,
    Mult,
    Divide,
    Subtract,
    Negate,
    LParen,
    RParen,
    Equals,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Mult => write!(f, "*"),
            Op::Divide => write!(f, "/"),
            Op::Subtract | Op::Negate => write!(f, "-"),
            Op::LParen => write!(f, "("),
            Op::RParen => write!(f, ")"),
            Op::Equals => write!(f, "="),
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
    Assignment {
        identifier: Rc<str>,
        value: Expr,
    }
}

#[derive(Debug)]
pub enum Expr {
    IntLiteral(i64),
    Identifier(Rc<str>),
    BinaryOp { 
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>
    },
    UnaryOp {
        op: Op,
        value: Box<Expr>,
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: vec![]
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

pub struct Parser {
    tokens: TokenStream
}

impl Parser {
    pub fn new() -> Self {
        let temp = TokenStream::new(vec![]);
        Self {
            tokens: temp,
        }
    }

    pub fn set_tokens(&mut self, tokens: TokenStream) {
        self.tokens = tokens;
    }

    pub fn parse(&mut self) -> Program {
        let mut ast = Program::new();

        loop {
            let current_token = self.tokens.next();
            
            match current_token {
                Token::KwPrint => {
                    let expr = self.parse_expression(0.0);
                    ast.add_statement(Statement::Print(expr));
                    continue;
                },
                Token::Eof => break,
                Token::Semicolon => continue,
                _ => panic!("Unexpected token: {current_token:#?}"),
            }
        }

        ast
    }

    pub fn parse_expression(&mut self, min_bp: f32) -> Expr {
        let next_token = self.tokens.next();
        let mut left = self.null_denotation(next_token);

        loop {
            let op = match self.tokens.peek() {
                Token::Semicolon => break,
                Token::RParen => break,
                Token::Plus => Op::Add,
                Token::Subtract => Op::Subtract,
                Token::Times => Op::Mult,
                Token::Divide => Op::Divide,
                t => panic!("Unexpected token: \"{t}\", expected infix operator"),
            };

            let (l_bp, r_bp) = infix_binding_power(op);

            if l_bp < min_bp {
                break;
            }

            self.tokens.next();

            let right = self.parse_expression( r_bp);
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        };

        left
    }

    fn null_denotation(&mut self, token: Token) -> Expr {
        match token {
            Token::IntLiteral(i) => Expr::IntLiteral(i),
            Token::Identifier(i) => Expr::Identifier(i),
            Token::Subtract => {
                let next_token = self.tokens.next();
                let value = self.null_denotation(next_token);
                Expr::UnaryOp { op: Op::Negate, value: Box::new(value) }
            }
            Token::LParen => {
                let lhs = self.parse_expression( 0.0);
                assert_eq!(self.tokens.next(), Token::RParen);
                lhs
            },
            t => panic!("Got \"{t}\', expected int"),
        }
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