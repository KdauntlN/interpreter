#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    IntLiteral(i64),
    Plus,
    Times,
    Divide,
    Subtract,
    LParen,
    RParen,
    Eof
}

pub struct Lexer {
    pub tokens: Vec<Token>
}

impl Lexer {
    pub fn tokenize(content: String) -> Lexer {
        let mut chars = content.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                let int = build_int(&mut chars, c);
                tokens.push(Token::IntLiteral(int));
            }

            tokens.push(match c {
                '+' => Token::Plus,
                '*' => Token::Times,
                '/' => Token::Divide,
                '-' => Token::Subtract,
                '(' => Token::LParen,
                ')' => Token::RParen,
                _ => continue,
            });
            continue;

        }

        tokens.reverse();
        Lexer { tokens }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
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