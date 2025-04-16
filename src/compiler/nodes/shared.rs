#[derive(Debug)]
pub enum ArithmethicOperator {
    Plus,
    Minus,
    Division,
    Multiply,
    Remainder,
}

#[derive(Debug)]
pub enum CompareOperator {
    Not,
    Compare,
    HigherThan,
    HigherThanOrEquals,
    LowerThan,
    LowerThanOrEquals,
}
