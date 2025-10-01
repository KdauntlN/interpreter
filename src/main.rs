use std::fs;
use clap::Parser;
use interpreter::Interpreter;

#[derive(Parser)]
pub struct Cli {
    pub filepath: String,
    #[arg(long)]
    pub dump_tokens: bool,
    #[arg(long)]
    pub dump_ast: bool,
}

fn main() {
    let cli = Cli::parse();
    let content = fs::read_to_string(cli.filepath).unwrap();

    let mut interpreter = Interpreter::new(&content);
    interpreter.build_ast();
}