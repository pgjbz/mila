use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub enum Precedence {
    Lowest = 1,
    AndOr = 2,
    Equals = 3,
    LessGreater = 4,
    Sum = 5,
    Product = 6,
    Prefix = 7,
    Call = 8,
    Index = 9,
}

impl PartialEq for Precedence {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (cur, other) if cur == other => Some(Ordering::Equal),
            (cur, other) if (*cur as usize) > (*other as usize) => Some(Ordering::Greater),
            (cur, other) if (*cur as usize) < (*other as usize) => Some(Ordering::Less),
            _ => Some(Ordering::Less),
        }
    }
}

#[macro_export]
macro_rules! precedence {
    ($val: expr) => {{
        use crate::lexer::token::token_type::TokenType;
        use crate::parser::precedence::Precedence;
        match $val {
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            TokenType::Or | TokenType::And => Precedence::AndOr,
            TokenType::Less
            | TokenType::Greater
            | TokenType::LessThanOrEq
            | TokenType::GreaterThanOrEq
            | TokenType::Assign => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus | TokenType::PlusAssign | TokenType::MinusAssign => {
                Precedence::Sum
            }
            TokenType::Slash
            | TokenType::Mod
            | TokenType::Asterisk
            | TokenType::BitWiseAnd
            | TokenType::Pipe
            | TokenType::ShiftLeft
            | TokenType::ShiftRight
            | TokenType::AsteriskAssign
            | TokenType::SlashAssign
            | TokenType::Caret => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            TokenType::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }};
}
