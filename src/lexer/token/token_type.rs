#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum TokenType {
    Or,
    Eq,
    Fn,
    Mod,
    Eof,
    Var,
    And,
    Ret,
    Dot,
    Let,
    Bang,
    True,
    Plus,
    Less,
    Pipe,
    NotEq,
    While,
    False,
    Caret,
    Comma,
    Colon,
    Slash,
    Minus,
    Number,
    Assign,
    LBrace,
    RBrace,
    PlusAssign,
    LParen,
    RParen,
    String,
    SlashAssign,
    Greater,
    MinusAssign,
    Illegal,
    LBracket,
    Question,
    RBracket,
    Asterisk,
    ShiftLeft,
    Semicolon,
    AsteriskAssign,
    Identifier,
    BitWiseAnd,
    ShiftRight,
    LessThanOrEq,
    GreaterThanOrEq,
    FloatingPointNumber,
}
