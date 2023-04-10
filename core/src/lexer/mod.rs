#![allow(dead_code)]
mod tokentype;
use tokentype::Token;
use tokentype::TokenType;
//create a lexer struct that uses peekable iterator for the source code
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
        while let Some(c) = self.next() {
            if c == '"' {
                break;
            }
            string.push(c);
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
        let mut identifier = String::from(c);
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == &'_' {
                identifier.push(self.next().unwrap());
            } else {
                break;
            }
        }
        identifier
    }
    pub fn lex(&mut self) {
        while self.peek() != None {
            use TokenType::*;
            let c = self.next().unwrap();
            match c {
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
                }
                _ => todo!(),
            }
        }
    }
}
