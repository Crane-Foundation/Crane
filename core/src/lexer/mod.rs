#![allow(dead_code)]
mod tokentype;
use std::process;
use std::collections::HashMap;
pub use tokentype::Token;
pub use tokentype::TokenType;
mod error;
use crate::throw;
use ansi_term::Colour::Red;
//create a lexer struct that uses peekable iterator for the source code
#[derive(Debug, Clone)]
pub struct Lexer {
    source: std::iter::Peekable<std::str::Chars<'static>>,
    line: usize,
    pub tokens: Vec<Token>,
}
impl Lexer {
    pub fn new(source: &'static str) -> Self {
        Self {
            source: source.chars().peekable(),
            line: 1,
            tokens: Vec::new(),
        }
    }
    fn next(&mut self) -> Option<char> {
        self.source.next().and_then(|c| (c != '\0').then_some(c))
    }
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }
    fn read_string(&mut self) -> String {
        let mut string = String::new();
        let mut last: char = '\0';
        while let Some(c) = self.next() {
            last = c;
            if c == '\0' {
                throw!("Unterminated string", self.line);
            }
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.line += 1;
            }
            if c == '\\' {
                let c = self.next().unwrap();
                match c {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '0' => string.push('\0'),
                    '"' => string.push('"'),
                    '\'' => string.push('\''),
                    '\\' => string.push('\\'),
                    _ => string.push(c),
                }
                continue;
            }
            string.push(c);
        }
        if last != '"' {
            throw!("Unterminated string", self.line);
        }
        string
    }
    fn read_number(&mut self, c: char) -> String {
        let mut number = String::from(c);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                number.push(self.next().unwrap());
            } else {
                break;
            }
        }
        number
    }
    fn read_identifier(&mut self, c: char) -> String {
        let line = self.line;
        let mut identifier = String::from(c);
        while let Some(c) = self.peek() {
            if c == &'('
                || c == &')'
                || c == &'{'
                || c == &'}'
                || c == &'['
                || c == &']'
                || c == &'<'
                || c == &'>'
                || c == &','
                || c == &';'
                || c == &'='
                || c == &':'
                || c == &'+'
                || c == &'-'
                || c == &'*'
                || c == &'/'
            {
                break;
            }
            if c.is_ascii_alphanumeric() || c == &'_' || c == &'.' {
                identifier.push(self.next().unwrap());
            }
            //if c is a symbol, throw an error
            else if c.is_ascii_punctuation() {
                if c == &'"' {
                    throw!("Unterminated String", line);
                }
                throw!(format!("Invalid character in identifier: '{}'", c), line);
            } else {
                break;
            }
        }
        identifier
    }
    fn read_char(&mut self) -> char {
        let c = self.next().unwrap();
        if c == '\\' {
            let c = self.next().unwrap();
            let f = match c {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '0' => '\0',
                '"' => '"',
                '\'' => '\'',
                '\\' => '\\',
                _ => {
                    throw!("Invalid escape character: {}", self.line);
                }
            };
            self.next();
            f
        } else {
            let f = self.next();
            if f.is_none() || f.unwrap() != '\'' {
                throw!(format!("Invalid character literal"), self.line);
            }
            c
        }
    }
    pub fn lex(&mut self) {
        while self.peek().is_some() {
            use TokenType::*;
            let c = self.next().unwrap();
            match c {
                '\n' => self.line += 1,
                '(' => self.tokens.push(Token::new(LeftParen, self.line)),
                ')' => self.tokens.push(Token::new(RightParen, self.line)),
                '{' => self.tokens.push(Token::new(LeftBrace, self.line)),
                '}' => self.tokens.push(Token::new(RightBrace, self.line)),
                ',' => self.tokens.push(Token::new(Comma, self.line)),
                '.' => self.tokens.push(Token::new(Dot, self.line)),
                '-' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("SubEq".to_string())
                        }
                        _ => Operator("Sub".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '+' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("AddEq".to_string())
                        }
                        _ => Operator("Add".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '*' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("MulEq".to_string())
                        }
                        _ => Operator("Mul".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '/' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("DivEq".to_string())
                        }
                        _ => Operator("Div".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '%' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("ModEq".to_string())
                        }
                        _ => Operator("Mod".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '^' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("PowEq".to_string())
                        }
                        _ => Operator("Pow".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '&' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("AndEq".to_string())
                        }
                        _ => Operator("And".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '|' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("OrEq".to_string())
                        }
                        _ => Operator("Or".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '!' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("NotEq".to_string())
                        }
                        _ => Operator("Not".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '=' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("EqEq".to_string())
                        }
                        _ => Operator("Eq".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '<' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("LessEq".to_string())
                        }
                        _ => Operator("Less".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '>' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            Operator("GreaterEq".to_string())
                        }
                        _ => Operator("Greater".to_string()),
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '"' => {
                    let string = self.read_string();
                    self.tokens.push(Token::new(Str(string), self.line));
                }
                ' ' | '\r' | '\t' => {}
                '1'..='9' => {
                    let number = self.read_number(c);
                    self.tokens.push(Token::new(Number(number), self.line));
                }
                'a'..='z' | 'A'..='Z' => {
                    let identifier = self.read_identifier(c);
                    match &*identifier {
                        "if" => self.tokens.push(Token::new(Keyword(identifier), self.line)),
                        "else" => self.tokens.push(Token::new(Keyword(identifier), self.line)),
                        "while" => self.tokens.push(Token::new(Keyword(identifier), self.line)),
                        "for" => self.tokens.push(Token::new(Keyword(identifier), self.line)),
                        "break" => self.tokens.push(Token::new(Keyword(identifier), self.line)),
                        "continue" => self.tokens.push(Token::new(Keyword(identifier), self.line)),
                        "return" => self.tokens.push(Token::new(Keyword(identifier), self.line)),
                        "def" => self.tokens.push(Token::new(Keyword(identifier), self.line)),
                        "let" => self.tokens.push(Token::new(Keyword(identifier), self.line)),
                        "True" => self.tokens.push(Token::new(True, self.line)),
                        "true" => self.tokens.push(Token::new(True, self.line)),
                        "False" => self.tokens.push(Token::new(False, self.line)),
                        "false" => self.tokens.push(Token::new(False, self.line)),
                        "None" => self.tokens.push(Token::new(None, self.line)),
                        _ => self
                            .tokens
                            .push(Token::new(Identifier(identifier), self.line)),
                    }
                }
                '\'' => {
                    let c = self.read_char();
                    self.tokens
                        .push(Token::new(Character(c.to_string()), self.line));
                }
                _ => {
                    throw!(format!("Unexpected character: {}", c), self.line);
                }
            }
        }
        self.tokens.push(Token::new(TokenType::Eof, self.line));
        check_for_unclosed_brackets(&self.tokens);
    }
}

//fn check_for_unclosed_brackets(tokens: &Vec<Token>) {
//this function checks for unclosed brackets, braces, and parentheses, error for every single unclosed bracket
//use a hashmap to store the line number of the opening bracket
//use another hashmap to store the line number of the closing bracket
//if the closing bracket is empty, throw an error for the opening bracket
//if the opening bracket is empty, throw an error for the closing bracketwuse std::collections::HashMap;
fn check_for_unclosed_brackets(tokens: &Vec<Token>) {
    let mut left_paren_count = 0;
    let mut right_paren_count = 0;
    let mut left_paren_lines = Vec::new();
    let mut right_paren_lines = Vec::new();

    let mut left_brace_count = 0;
    let mut right_brace_count = 0;
    let mut left_brace_lines = Vec::new();
    let mut right_brace_lines = Vec::new();

    let mut err: Result<(), ()> = Ok(());

    for token in tokens {
        match token.token_type {
            TokenType::LeftParen => {
                left_paren_count += 1;
                left_paren_lines.push(token.line);
            }
            TokenType::RightParen => {
                right_paren_count += 1;
                right_paren_lines.push(token.line);
            }
            TokenType::LeftBrace => {
                left_brace_count += 1;
                left_brace_lines.push(token.line);
            }
            TokenType::RightBrace => {
                right_brace_count += 1;
                right_brace_lines.push(token.line);
            }
            _ => {}
        }
    }

    // Check for unclosed parentheses
    if left_paren_count != right_paren_count {
        let unclosed_count = left_paren_count - right_paren_count;
        if unclosed_count > 0 {
            for _ in 0..unclosed_count {
                if let Some(line) = left_paren_lines.pop() {
                    throw!(format!("Unclosed '(' at line {}", line), line, false);
                    err = Err(());
                }
            }
        } else {
            for _ in 0..-unclosed_count {
                if let Some(line) = right_paren_lines.pop() {
                    throw!(format!("Unclosed ')' at line {}", line), line, false);
                    err = Err(());
                }
            }
        }
    }

    // Check for unclosed braces
    if left_brace_count != right_brace_count {
        let unclosed_count = left_brace_count - right_brace_count;
        if unclosed_count > 0 {
            for _ in 0..unclosed_count {
                if let Some(line) = left_brace_lines.pop() {
                    throw!(format!("Unclosed '{{' at line {}", line), line, false);
                    err = Err(());
                }
            }
        } else {
            for _ in 0..-unclosed_count {
                if let Some(line) = right_brace_lines.pop() {
                    throw!(format!("Unclosed '}}' at line {}", line), line, false);
                    err = Err(());
                }
            }
        }
    }

    if err == Err(()) {
        process::exit(0);
    }
}
