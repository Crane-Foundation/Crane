#[macro_use]
mod lexer;
mod parser;
use std::fs;

fn main() {
    let source = fs::read_to_string("main.crane").unwrap();
    let source: &'static str = Box::leak(source.into_boxed_str());
    let mut lexer = lexer::Lexer::new(source.clone());
    lexer.lex();
    let mut parser = parser::Parser::new(lexer.tokens);
    parser.parse();
    println!("{:#?}", parser.tree);
}
