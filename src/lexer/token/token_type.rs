use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum TokenType {
    Or,
    Eq,
    If,
    Fn,
    Mod,
    Eof,
    Var,
    And,
    Ret,
    Dot,
    Let,
    Else,
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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match self {
            Self::Or => "||",
            Self::If => "if",
            Self::Eq => "==",
            Self::Fn => "fn",
            Self::Mod => "%",
            Self::Dot => ".",
            Self::Bang => "!",
            Self::Plus => "+",
            Self::Less => "<",
            Self::Pipe => "|",
            Self::Else => "else",
            Self::Caret => "^",
            Self::Eof => "eof",
            Self::Var => "var",
            Self::Comma => ",",
            Self::Colon => ":",
            Self::And => "and",
            Self::Slash => "/",
            Self::Ret => "ret",
            Self::Let => "let",
            Self::Minus => "-",
            Self::Assign => "=",
            Self::LBrace => "{",
            Self::RBrace => "}",
            Self::LParen => "(",
            Self::RParen => ")",
            Self::NotEq => "!=",
            Self::True => "true",
            Self::Greater => ">",
            Self::LBracket => "[",
            Self::Question => "?",
            Self::RBracket => "]",
            Self::Asterisk => "*",
            Self::False => "false",
            Self::While => "while",
            Self::Semicolon => ";",
            Self::ShiftLeft => "<<",
            Self::BitWiseAnd => "&",
            Self::ShiftRight => ">>",
            Self::PlusAssign => "+=",
            Self::Number => "number",
            Self::String => "string",
            Self::SlashAssign => "/=",
            Self::MinusAssign => "-=",
            Self::Illegal => "illegal",
            Self::LessThanOrEq => "<=",
            Self::AsteriskAssign => "*=",
            Self::GreaterThanOrEq => ">=",
            Self::Identifier => "identifier",
            Self::FloatingPointNumber => "float number",
        };
        write!(f, "{}", literal)
    }
}
