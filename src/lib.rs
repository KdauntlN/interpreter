pub mod lexing;
pub mod parsing;

use lexing::{
    Lexer,
    Token
};

use parsing::{
    Parser,
};

pub struct Interpreter<'a> {
    lexer: Lexer<'a>,
    parser: Parser,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {

        let lexer = Lexer::new(source);
        let parser = Parser::new();

        Self {
            lexer,
            parser
        }
    }

    pub fn build_ast(&mut self) {
        let tokens = self.lexer.tokenize();
        let token_stream = TokenStream::new(tokens);

        self.parser.set_tokens(token_stream);
        self.parser.parse();
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