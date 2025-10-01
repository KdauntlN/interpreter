use crate::{
    lexing::Token,
    TokenStream
};

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

#[derive(Debug)]
pub enum Statement {
    Print(Expr)
}

#[derive(Debug)]
pub enum Expr {
    IntLiteral(i64),
    BinaryOp { 
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>
    }
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

pub struct Parser {
    tokens: TokenStream,
    ast: Program
}

impl Parser {
    pub fn new() -> Self {
        let temp = TokenStream::new(vec![]);
        Self {
            tokens: temp,
            ast: Program {
                statements: vec![]
            }
        }
    }

    pub fn set_tokens(&mut self, tokens: TokenStream) {
        self.tokens = tokens;
    }

    pub fn parse(&mut self) {
        loop {
            let current_token = self.tokens.next();
            
            match current_token {
                Token::KwPrint => {
                    let expr = self.parse_expression(0.0);
                    self.ast.add_statement(Statement::Print(expr));
                    continue;
                },
                Token::Eof => break,
                Token::Semicolon => continue,
                _ => panic!("Unexpected token: {current_token:#?}"),
            }
        }

        println!("{:#?}", self.ast);
    }

    pub fn parse_expression(&mut self, min_bp: f32) -> Expr {
        let mut left = match self.tokens.next() {
            Token::IntLiteral(i) => Expr::IntLiteral(i),
            Token::LParen => {
                let lhs = self.parse_expression( 0.0);
                assert_eq!(self.tokens.next(), Token::RParen);
                lhs
            },
            t => panic!("Got \"{t}\', expected int"),
        };

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