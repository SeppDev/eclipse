use std::fmt::Display;

use crate::compiler::lexer::token::TokenKind;

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmethicOperator {
    Plus,
    Subtract,
    Division,
    Multiply,
    Remainder,
    LeftBitshift,
    RightBitshift,
}
impl Display for ArithmethicOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ArithmethicOperator::*;

        write!(
            f,
            "{}",
            match self {
                Plus => "+",
                Subtract => "-",
                Division => "/",
                Multiply => "*",
                Remainder => "%",
                RightBitshift => ">>",
                LeftBitshift => "<<",
            }
        )
    }
}

impl Into<ArithmethicOperator> for TokenKind {
    fn into(self) -> ArithmethicOperator {
        use TokenKind::*;

        match self {
            Plus => ArithmethicOperator::Plus,
            Minus => ArithmethicOperator::Subtract,
            ForwardSlash => ArithmethicOperator::Division,
            Asterisk => ArithmethicOperator::Multiply,
            Percent => ArithmethicOperator::Remainder,
            _ => panic!("Invalid token for operator conversion: {self:?}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EqualsOperation {
    Equals,
    PlusEquals,
    SubtractEquals,
    MultiplyEquals,
    DivideEquals,
    RemainderEquals,
}
impl Display for EqualsOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EqualsOperation::*;

        write!(
            f,
            "{}",
            match self {
                Equals => "=",
                PlusEquals => "+=",
                SubtractEquals => "-=",
                MultiplyEquals => "*=",
                DivideEquals => "/=",
                RemainderEquals => "%=",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompareOperator {
    Not,
    Compare,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
    And,
    Or,
}
impl Display for CompareOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CompareOperator::*;

        write!(
            f,
            "{}",
            match self {
                Not => "!=",
                Compare => "==",
                HigherThan => ">",
                HigherThanOrEquals => ">=",
                LowerThan => "<",
                LowerThanOrEquals => "<=",
            }
        )
    }
}
impl Into<CompareOperator> for TokenKind {
    fn into(self) -> CompareOperator {
        use TokenKind::*;

        match self {
            Compare => CompareOperator::Compare,
            GreaterThan => CompareOperator::GreaterThan,
            GreaterThanOrEquals => CompareOperator::GreaterThanOrEquals,
            LessThan => CompareOperator::LessThan,
            LessThanOrEquals => CompareOperator::LessThanOrEquals,
            NotEquals => CompareOperator::Not,
            And => CompareOperator::And,
            Or => CompareOperator::Or,
            _ => panic!("Invalid token for operator conversion: {self:?}"),
        }
    }
}
