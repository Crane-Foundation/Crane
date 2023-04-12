#[macro_use]
mod lexer;
use std::fs;
fn main() {
    let source = fs::read_to_string("main.crane").unwrap();
    let source: &'static str = Box::leak(source.into_boxed_str());
    let mut lexer = lexer::Lexer::new(source.clone());
    lexer.lex();
    for token in lexer.tokens {
        println!("{:?}", token.token_type);
    }
}