mod lexer;
fn main() {
    let source = "12";
    let mut lexer = lexer::Lexer::new(source);
    lexer.lex();
    for token in lexer.tokens {
        println!("{:?}", token);
    }
}
