#![allow(unused_imports, dead_code, unused_variables, private_bounds, private_interfaces, unused_mut)]
mod lexer;
use lexer::error;
use lexer::lex;
fn main() {
    let input = "2.4\n3.5";
    let mut lexer = lex::Lexer::new(input);
    let tk = lexer.lex();
    println!("{:?}", tk);
}
