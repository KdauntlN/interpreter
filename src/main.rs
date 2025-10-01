use interpreter::{Cli,
    lexer::Lexer,
    parse_expression
};

use std::fs;
use clap::Parser;
use display_tree::{format_tree, CharSet, Style, StyleBuilder};

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