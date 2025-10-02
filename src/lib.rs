pub mod lexing;
pub mod parsing;
pub mod error;

use lexing::{
    Lexer,
    Token,
    TokenKind
};

use parsing::{
    Parser,
    Program,
    Statement,
    Expr,
    Op
};

use std::{
    process,
    rc::Rc,
    str::Lines
};

pub struct Interpreter<'a> {
    _filename: Rc<str>,
    lines: Lines<'a>,
    lexer: Lexer<'a>,
    parser: Parser,
    ast: Program
}

impl<'a> Interpreter<'a> {
    pub fn new(filename: Rc<str>, source: &'a str) -> Self {
        let lines = source.lines();

        let lexer = Lexer::new(source);
        let parser = Parser::new(Rc::clone(&filename));
        let ast = Program::new();

        Self {
            _filename: filename,
            lines,
            lexer,
            parser,
            ast
        }
    }

    pub fn build_ast(&mut self) {
        let (tokens, line_number, col) = self.lexer.tokenize();
        let token_stream = TokenStream::new(tokens, line_number, col);

        self.parser.set_tokens(token_stream);
        self.ast = self.parser.parse().unwrap_or_else(|err| {
            let error = err.format_err(self.lines.nth((err.line - 1) as usize).unwrap());
            eprintln!("error: {error}");
            process::exit(1);
        });
    }

    pub fn run(&mut self) {
        for statement in &mut self.ast.statements {
            match statement {
                Statement::Print(expr) => {
                    let result = evaluate_expr(expr);
                    println!("{result}");
                },
                _ => panic!("I'll get there hold on"),
            }
        }
    }
}

fn evaluate_expr(expr: &mut Expr) -> i64 {
    match expr {
        Expr::IntLiteral(n) => *n,
        Expr::BinaryOp { op, left, right } => {
            match op {
                Op::Add => {
                    let left = evaluate_expr(left);
                    let right = evaluate_expr(right);
                    left + right
                },
                Op::Divide => {
                    let left = evaluate_expr(left);
                    let right = evaluate_expr(right);
                    left / right
                },
                Op::Subtract => {
                    let left = evaluate_expr(left);
                    let right = evaluate_expr(right);
                    left - right
                },
                Op::Mult => {
                    let left = evaluate_expr(left);
                    let right = evaluate_expr(right);
                    left * right
                },
                _ => unreachable!(),
            }
        },
        Expr::UnaryOp { op, value } => {
            let value = evaluate_expr(value);
            match op {
                Op::Negate => value * -1,
                _ => panic!("Unexpected token"),
            }
        }
        _ => panic!("Not implemented"),
    }
}

pub struct TokenStream {
    tokens: Box<[Token]>,
    index: usize,
    line_number: i32,
    col: i32
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>, line_number: i32, col: i32) -> Self {
        Self {
            tokens: Box::from(tokens),
            index: 0,
            line_number,
            col
        }
    }

    pub fn next(&mut self) -> Token {
        let token = self.tokens
            .get(self.index)
            .cloned()
            .unwrap_or(Token::new(TokenKind::Eof, self.line_number, self.col, 1));

        self.index += 1;
        token
    }

    pub fn peek(&mut self) -> Token {
        self.tokens
            .get(self.index)
            .cloned()
            .unwrap_or(Token::new(TokenKind::Eof, self.line_number, self.col, 1))
    }
}