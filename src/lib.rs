pub mod lexing;
pub mod parsing;

use lexing::{
    Lexer,
    Token
};

use parsing::{
    Parser,
    Program,
    Statement,
    Expr,
    Op
};

pub struct Interpreter<'a> {
    lexer: Lexer<'a>,
    parser: Parser,
    ast: Program
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {

        let lexer = Lexer::new(source);
        let parser = Parser::new();
        let ast = Program::new();

        Self {
            lexer,
            parser,
            ast
        }
    }

    pub fn build_ast(&mut self) {
        let tokens = self.lexer.tokenize();
        let token_stream = TokenStream::new(tokens);

        self.parser.set_tokens(token_stream);
        self.ast = self.parser.parse();
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
        _ => panic!("Not implemented"),
    }
}

pub struct TokenStream {
    tokens: Box<[Token]>,
    index: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: Box::from(tokens),
            index: 0
        }
    }

    pub fn next(&mut self) -> Token {
        let token = self.tokens
            .get(self.index)
            .cloned()
            .unwrap_or(Token::Eof);

        self.index += 1;
        token
    }

    pub fn peek(&mut self) -> Token {
        self.tokens
            .get(self.index)
            .cloned()
            .unwrap_or(Token::Eof)
    }
}