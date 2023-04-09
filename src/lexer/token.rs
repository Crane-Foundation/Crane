#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    //Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Ampersand,
    Pipe,
    Tilde,
    Bang,
    Equal,
    Less,
    Greater,
    Question,
    Colon,
    Dot,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    //Keywords
    Keyword,
    //Literals
    Identifier,
    Integer,
    Float,
    String,
    Char,
    //Misc
    Eof,
    Error,
    //Modifiers
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    PowEq,
    AndEq,
    OrEq,
    XorEq,
    ShlEq,
    ShrEq,
    EqEq,
    NotEq,
    LessEq,
    GreaterEq,
    AndAnd,
    OrOr,
    Shl,
    Shr,
    Arrow,
    FatArrow,
    DotDot,
    DotDotEq,
    DotDotDot,
    At,
    Pound,
    Dollar,
    ColonColon,
    //Whitespace
    Comment,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}