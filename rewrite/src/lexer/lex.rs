use super::error::ErrorType;
use crate::error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) enum TokenType {
    // Operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow, // + - * / % ^
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge, // == != < <= > >=
    // Logical
    And,
    Or,
    Not, // & | !
    // Bitwise
    BitAnd,
    BitOr,
    BitNot,
    ShL,
    ShR, // & | ~ << >>
    // Assignment
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign, // = += -= *= /= %= ^=
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Colon,
    DoubleColon,
    Semicolon, // ( ) { } [ ] , . : :: ;
    LAngle,
    RAngle, // < >
    // Literals
    Int,
    Float,
    String,
    Type,
    Identifier, // 123 123.456 "abc" type ident
    // Keywords
    Keyword, // In a hashset
    // End of file
    Eof,
}

struct Number;
struct Identifier;
struct Operator;

struct Eof;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct Token {
    tp: TokenType,
    val: String,
    line: usize,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    context: Context<'a>,
}

#[derive(Debug, Clone)]
struct Context<'a> {
    tokens: Vec<Token>,
    input: &'a str,
    pos: usize,
    line: usize,
    lines: Vec<&'a str>,
}

pub trait Lex {
    fn lex(&self, ctx: &mut Context) -> Option<Token>;
}

// Implementations

impl<'a> Context<'a> {
    fn new(input: &'a str, pos: usize) -> Self {
        Self {
            tokens: Vec::new(),
            input,
            pos,
            line: 0,
            lines: input.lines().collect(),
        }
    }
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }
    fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        self.peek()
    }
    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer {
        Lexer {
            input: s,
            pos: 0,
            context: Context::new(s, 0),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }
    pub fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        self.context.pos += 1;
        self.peek()
    }
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            if let Some(c) = self.context.peek() {
                if c == '\n' {
                    self.context.line += 1;
                    self.advance();
                } else if c == ' ' {
                    self.advance();
                    continue;
                }
            }
            let lex: &dyn Lex =
                Lexer::decide(self.context.input[self.context.pos..].chars().next());
            let f = lex.lex(&mut self.context);
            // Add logic to break the loop when done
            match f {
                Some(token) => {
                    tokens.push(token);
                }
                None => break,
            }
        }
        tokens
    }

    pub fn decide<'d>(c: Option<char>) -> &'d dyn Lex {
        match c {
            Some(c) => match c {
                '0'..='9' => &Number,
                'a'..='z' | 'A'..='Z' => &Identifier,
                '+' | '-' | '*' | '/' => &Operator,
                _ => error!(
                    format!("Unexpected Token '{}'", c),
                    0,
                    ErrorType::UnexpectedToken,
                    "",
                    c
                ),
            },
            None => &Eof,
        }
    }
}

impl Lex for Number {
    fn lex(&self, ctx: &mut Context) -> Option<Token> {
        let mut num = String::new();
        let start = ctx.pos;
        while ctx.advance().map_or(false, |c| c.is_numeric() || c == '.') {}
        if ctx.pos == start {
            return None;
        }
        Some(Token {
            tp: TokenType::Int,
            val: ctx.input[start..ctx.pos].to_string(),
            line: ctx.line,
        })
    }
}

impl Lex for Identifier {
    fn lex(&self, ctx: &mut Context) -> Option<Token> {
        let mut ident = String::new();
        let start = ctx.pos;
        while ctx
            .advance()
            .map_or(false, |c| c.is_alphanumeric() || c == '_')
        {}
        if ctx.pos == start {
            return None;
        }
        Some(Token {
            tp: TokenType::Identifier,
            val: ctx.input[start..ctx.pos].to_string(),
            line: ctx.line,
        })
    }
}

impl Lex for Operator {
    fn lex(&self, ctx: &mut Context) -> Option<Token> {
        // Decide which operator it is
        match ctx.peek().unwrap_or('\0') {
            '+' => {
                ctx.advance();
                Some(Token {
                    tp: TokenType::Add,
                    val: "+".to_string(),
                    line: ctx.line,
                })
            }
            _ => error!(
                format!("Unexpected character '{}'", ctx.peek().unwrap_or('\0')),
                ctx.line,
                ErrorType::UnexpectedCharacter,
                ctx.lines[ctx.line],
                ctx.peek().unwrap()
            ),
        }
    }
}

impl Lex for Eof {
    fn lex(&self, ctx: &mut Context) -> Option<Token> {
        None
    }
}
