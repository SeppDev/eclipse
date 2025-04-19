use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmethicOperator {
    Plus,
    Subtract,
    Division,
    Multiply,
    Remainder,
}
impl Display for ArithmethicOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ArithmethicOperator::*;

        write!(f, "{}", match self {
            Plus => "+",
            Subtract => "-",
            Division => "/",
            Multiply => "*",
            Remainder => "%"
        })
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

        write!(f, "{}", match self {
            Not => "!=",
            Compare => "==",
            HigherThan => ">",
            HigherThanOrEquals => ">=",
            LowerThan => "<",
            LowerThanOrEquals => "<="
        })
    }
}
