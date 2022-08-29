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
    Dot = 8,
    Call = 9,
    Index = 10,
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
        use crate::lexer::TokenType::*;
        use crate::parser::precedence::Precedence;
        match $val {
            Eq | NotEq => Precedence::Equals,
            Or | And => Precedence::AndOr,
            Less
            | Greater
            | LessThanOrEq
            | GreaterThanOrEq
            | Assign => Precedence::LessGreater,
            Plus | Minus | PlusAssign | MinusAssign => {
                Precedence::Sum
            }
            Slash
            | Mod
            | Asterisk
            | BitWiseAnd
            | Pipe
            | ShiftLeft
            | ShiftRight
            | AsteriskAssign
            | SlashAssign
            | Caret => Precedence::Product,
            LParen => Precedence::Call,
            LBracket => Precedence::Index,
            Dot => Precedence::Dot,
            _ => Precedence::Lowest,
        }
    }};
}
