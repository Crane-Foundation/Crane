#![allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types, dead_code)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,

    // Literals.
    Operator(String),
    Identifier(String),
    True,
    False,
    None,
    Str(String),
    Number(String),
    Character(String),
    Keyword(String),
    DataType(String),
    Eof,
}
impl TokenType {
    pub fn unwrap(&self) -> Option<String> {
        match self.clone() {
            TokenType::Identifier(s) => Some(s),
            TokenType::Str(s) => Some(s),
            TokenType::Number(s) => Some(s),
            TokenType::Character(s) => Some(s),
            TokenType::Keyword(s) => Some(s),
            TokenType::DataType(s) => Some(s),
            TokenType::Operator(s) => Some(s),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Self {
        Self { token_type, line }
    }
    pub fn as_string(&self) -> String {
        match self.token_type {
            TokenType::LeftParen => "LeftParen".to_string(),
            TokenType::RightParen => "RightParen".to_string(),
            TokenType::LeftBrace => "LeftBrace".to_string(),
            TokenType::RightBrace => "RightBrace".to_string(),
            TokenType::Comma => "Comma".to_string(),
            TokenType::Dot => "Dot".to_string(),
            TokenType::Operator(ref s) => format!("{}", s),
            TokenType::Identifier(ref s) => format!("{}", s),
            TokenType::True => "True".to_string(),
            TokenType::False => "False".to_string(),
            TokenType::None => "None".to_string(),
            TokenType::Str(ref s) => format!("{}", s),
            TokenType::Number(ref s) => format!("{}", s),
            TokenType::Character(ref s) => format!("{}", s),
            TokenType::Keyword(ref s) => format!("{}", s),
            TokenType::DataType(ref s) => format!("{}", s),
            TokenType::Eof => "Eof".to_string(),
        }
    }
}
