#[derive(Debug, Clone)]
pub struct Token {
    pub typ: Type,
    pub val: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Type {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    SemiColon,
    Plus,
    Multiply,
    Divide,
    Greater,
    Less,

    Assign,
    Equal,

    Minus,

    Input,
    Print,
    Var,
    If,
    While,

    Identifier,
    Number,
    StringLiteral,

    EOF,

    LexerError
}
