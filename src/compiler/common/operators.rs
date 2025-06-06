use std::fmt::Display;

use crate::compiler::lexer::token::TokenKind;

#[derive(Debug)]
pub struct ConversionError;

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

impl TryFrom<&TokenKind> for ArithmethicOperator {
    type Error = ConversionError;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        use TokenKind::*;

        Ok(match value {
            Plus => ArithmethicOperator::Plus,
            Minus => ArithmethicOperator::Subtract,
            ForwardSlash => ArithmethicOperator::Division,
            Asterisk => ArithmethicOperator::Multiply,
            Percent => ArithmethicOperator::Remainder,
            LeftBitshift => ArithmethicOperator::LeftBitshift,
            RightBitshift => ArithmethicOperator::RightBitshift,
            _ => return Err(ConversionError),
        })
    }
}
impl TryFrom<TokenKind> for ArithmethicOperator {
    type Error = ConversionError;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        (&value).try_into()
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
                Or => "||",
                And => "&&",
                Not => "!=",
                Compare => "==",
                GreaterThan => ">",
                GreaterThanOrEquals => ">=",
                LessThan => "<",
                LessThanOrEquals => "<=",
            }
        )
    }
}
impl TryFrom<&TokenKind> for CompareOperator {
    type Error = ConversionError;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        use TokenKind::*;

        Ok(match value {
            Compare => CompareOperator::Compare,
            GreaterThan => CompareOperator::GreaterThan,
            GreaterThanOrEquals => CompareOperator::GreaterThanOrEquals,
            LessThan => CompareOperator::LessThan,
            LessThanOrEquals => CompareOperator::LessThanOrEquals,
            NotEquals => CompareOperator::Not,
            And => CompareOperator::And,
            Or => CompareOperator::Or,
            _ => return Err(ConversionError),
        })
    }
}
impl TryFrom<TokenKind> for CompareOperator {
    type Error = ConversionError;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}
