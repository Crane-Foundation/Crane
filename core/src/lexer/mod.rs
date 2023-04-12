#![allow(dead_code)]
mod tokentype;
use tokentype::Token;
use tokentype::TokenType;
mod error;
use ansi_term::Colour::Red;
use crate::throw;
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
            last = c.clone();
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
            if c == &'(' {
                break;
            }
            if c.is_ascii_alphanumeric() || c == &'_' || c == &'.' {
                identifier.push(self.next().unwrap());
            }
            //if c is a symbol, throw an error
            else if c.is_ascii_punctuation() {
                throw!(format!("Invalid character in identifier: '{}'", c), line);
            }
             else {
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
            if f == None || f.unwrap() != '\'' {
                throw!(format!("Invalid character literal"), self.line);
            }
            c
        }
    }
    pub fn lex(&mut self) {
        while self.peek() != None {
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
                            SubEq
                        }
                        _ => Sub,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                }
                '+' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            AddEq
                        }
                        _ => Add,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '*' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            MulEq
                        }
                        _ => Mul,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '/' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            DivEq
                        }
                        _ => Div,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '%' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            ModEq
                        }
                        _ => Mod,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '^' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            PowEq
                        }
                        _ => Pow,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '&' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            AndEq
                        }
                        _ => And,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '|' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            OrEq
                        }
                        _ => Or,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '!' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            BangEqual
                        }
                        _ => Bang,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '=' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            EqualEqual
                        }
                        _ => Equal,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '<' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            LessEqual
                        }
                        _ => Less,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '>' => {
                    let token_type = match self.peek() {
                        Some(&'=') => {
                            self.next();
                            GreaterEqual
                        }
                        _ => Greater,
                    };
                    self.tokens.push(Token::new(token_type, self.line));
                },
                '"' => {
                    let string = self.read_string();
                    self.tokens.push(Token::new(Str(string), self.line));
                }
                ' ' | '\r' | '\t' => {}
                '1'..='9' => {
                    let number = self.read_number(c);
                    self.tokens.push(Token::new(Number(number), self.line));
                },
                'a'..='z' | 'A'..='Z' => {
                    let identifier = self.read_identifier(c);
                    self.tokens.push(Token::new(Identifier(identifier), self.line));
                },
                '\'' => {
                    let c = self.read_char();
                    self.tokens.push(Token::new(Character(c.to_string()), self.line));
                },
                _ => {throw!(format!("Unexpected character: {}", c), self.line);}
            }
        }
    }
}
