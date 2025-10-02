use crate::{
    lexing::{
        Token,
        TokenKind
    },
    error::Error,
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
    filename: Rc<str>,
    tokens: TokenStream
}

impl Parser {
    pub fn new(filename: Rc<str>) -> Self {
        let temp = TokenStream::new(vec![], 0, 0);
        Self {
            filename,
            tokens: temp,
        }
    }

    pub fn set_tokens(&mut self, tokens: TokenStream) {
        self.tokens = tokens;
    }

    pub fn parse(&mut self) -> Result<Program, Error> {
        let mut ast = Program::new();

        loop {
            let current_token = self.tokens.next();
            
            match &current_token.kind {
                TokenKind::KwPrint => {
                    let expr = self.parse_expression(0.0)?;
                    ast.add_statement(Statement::Print(expr));
                    continue;
                },
                TokenKind::Eof => break,
                TokenKind::Semicolon => continue,
                _ => return Err(Error::wrong_token(self.filename.clone(), current_token)),
            }
        }

        Ok(ast)
    }

    pub fn parse_expression(&mut self, min_bp: f32) -> Result<Expr, Error> {
        let next_token = self.tokens.next();
        let mut left = self.null_denotation(next_token)?;

        loop {
            let next_token = self.tokens.peek();
            let op = match next_token.kind {
                TokenKind::Semicolon => break,
                TokenKind::RParen => break,
                TokenKind::Plus => Op::Add,
                TokenKind::Subtract => Op::Subtract,
                TokenKind::Times => Op::Mult,
                TokenKind::Divide => Op::Divide,
                _ => return Err(Error::wrong_token(self.filename.clone(), next_token)),
            };

            let (l_bp, r_bp) = infix_binding_power(op);

            if l_bp < min_bp {
                break;
            }

            self.tokens.next();

            let right = self.parse_expression( r_bp)?;
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        };

        Ok(left)
    }

    fn null_denotation(&mut self, token: Token) -> Result<Expr, Error> {
        match &token.kind {
            TokenKind::IntLiteral(i) => Ok(Expr::IntLiteral(*i)),
            TokenKind::Identifier(i) => Ok(Expr::Identifier(i.clone())),
            TokenKind::Subtract => {
                let next_token = self.tokens.next();
                let value = self.null_denotation(next_token)?;
                Ok(Expr::UnaryOp { op: Op::Negate, value: Box::new(value) })
            }
            TokenKind::LParen => {
                let lhs = self.parse_expression(0.0)?;
                assert_eq!(self.tokens.next().kind, TokenKind::RParen);
                Ok(lhs)
            },
            _ => Err(Error::wrong_token(self.filename.clone(), token)),
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