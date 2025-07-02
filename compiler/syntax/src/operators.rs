use std::fmt::Display;

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
