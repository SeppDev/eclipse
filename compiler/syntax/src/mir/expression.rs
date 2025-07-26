use super::Type;

#[derive(Debug)]
pub enum Expression {
    Integer(Type, String),
}
