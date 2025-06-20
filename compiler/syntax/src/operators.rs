use std::fmt::Display;

use crate::token::TokenKind;

#[derive(Debug)]
pub struct ConversionError;

#[derive(Debug, PartialEq, Eq)]
pub enum CompareOperator {
    NotEquals,
    Compare,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
    And,
    Or,
}
impl TryFrom<&TokenKind> for CompareOperator {
    type Error = ConversionError;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        use TokenKind::*;

        let value = match value {
            NotEquals => CompareOperator::NotEquals,
            Compare => CompareOperator::Compare,
            GreaterThan => CompareOperator::GreaterThan,
            LessThan => CompareOperator::LessThan,
            LessThanOrEquals => CompareOperator::LessThanOrEquals,
            And => CompareOperator::And,
            Or => CompareOperator::Or,

            _ => return Err(ConversionError),
        };

        Ok(value)
    }
}
impl TryFrom<TokenKind> for CompareOperator {
    type Error = ConversionError;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticOperator {
    Plus,
    Subtract,
    Division,
    Multiply,
    Remainder,
    LeftBitshift,
    RightBitshift,
}
impl TryFrom<&TokenKind> for ArithmeticOperator {
    type Error = ConversionError;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        use TokenKind::*;

        let value = match value {
            Plus => ArithmeticOperator::Plus,
            Minus => ArithmeticOperator::Subtract,
            Asterisk => ArithmeticOperator::Multiply,
            ForwardSlash => ArithmeticOperator::Division,
            _ => return Err(ConversionError),
        };

        Ok(value)
    }
}
impl TryFrom<TokenKind> for ArithmeticOperator {
    type Error = ConversionError;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        Self::try_from(&value)
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

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Compare(CompareOperator),
}
impl TryFrom<&TokenKind> for Operator {
    type Error = ConversionError;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        if let Ok(op) = ArithmeticOperator::try_from(value) {
            return Ok(Operator::Arithmetic(op));
        }

        if let Ok(op) = CompareOperator::try_from(value) {
            return Ok(Operator::Compare(op));
        }

        Err(ConversionError)
    }
}
impl TryFrom<TokenKind> for Operator {
    type Error = ConversionError;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl Display for Operator {
    #[allow(unused, non_snake_case)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operator::*;

        write!(
            f,
            "{}",
            match self {
                Arithmetic(op) => format!("{op}"),
                Compare(op) => format!("{op}"),
            }
        )
    }
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

impl Display for ArithmeticOperator {
    #[allow(unused, non_snake_case)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ArithmeticOperator::*;

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

impl Display for CompareOperator {
    #[allow(unused, non_snake_case)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CompareOperator::*;

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
