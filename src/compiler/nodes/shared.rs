use std::fmt::Display;

use crate::compiler::lexer::token::{TokenInfo, TokenKind};

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

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Arithmetic(ArithmethicOperator),
    Comparison(CompareOperator),
}
impl Into<Operator> for ArithmethicOperator {
    fn into(self) -> Operator {
        Operator::Arithmetic(self)
    }
}
impl Into<Operator> for CompareOperator {
    fn into(self) -> Operator {
        Operator::Comparison(self)
    }
}
impl Into<Operator> for TokenInfo {
    fn into(self) -> Operator {
        use TokenKind::*;
        use Operator::*;
        
        match self.kind {
            Plus => Arithmetic(ArithmethicOperator::Plus),
            Minus => Arithmetic(ArithmethicOperator::Subtract),
            ForwardSlash => Operator::Arithmetic(ArithmethicOperator::Division),
            Asterisk => Operator::Arithmetic(ArithmethicOperator::Multiply),
            Percent => Operator::Arithmetic(ArithmethicOperator::Remainder),
            // Greater => Operator::Comparison(CompareOperator::GreaterThan),
            // Less => Operator::Comparison(CompareOperator::LessThan),
            // GreaterEqual => Operator::Comparison(CompareOperator::GreaterEqual),
            // LessEqual => Operator::Comparison(CompareOperator::LessEqual),
            // NotEqual => Operator::Comparison(CompareOperator::NotEqual),
            _ => panic!("Invalid token for operator conversion"),
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
    HigherThan,
    HigherThanOrEquals,
    LowerThan,
    LowerThanOrEquals,
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
