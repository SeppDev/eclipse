use std::fmt::Display;

use crate::lexer::token::TokenKind;

#[derive(Debug)]
pub struct ConversionError;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Subtract,
    Division,
    Multiply,
    Remainder,
    LeftBitshift,
    RightBitshift,

    Not,
    Compare,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
    And,
    Or,
}

impl Display for Operator {
    #[allow(unused, non_snake_case)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operator::*;

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

impl TryFrom<&TokenKind> for Operator {
    type Error = ConversionError;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        use TokenKind::*;

        #[allow(non_snake_case)]
        Ok(match value {
            Plus => Operator::Plus,
            Minus => Operator::Subtract,
            ForwardSlash => Operator::Division,
            Asterisk => Operator::Multiply,
            Percent => Operator::Remainder,
            LeftBitshift => Operator::LeftBitshift,
            RightBitshift => Operator::RightBitshift,

            Compare => Operator::Compare,
            GreaterThan => Operator::GreaterThan,
            GreaterThanOrEquals => Operator::GreaterThanOrEquals,
            LessThan => Operator::LessThan,
            LessThanOrEquals => Operator::LessThanOrEquals,
            NotEquals => Operator::Not,
            And => Operator::And,
            Or => Operator::Or,
            _ => return Err(ConversionError),
        })
    }
}
impl TryFrom<TokenKind> for Operator {
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
