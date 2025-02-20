use crate::common::position::Located;

use super::{
    ast::{Identifier, Parameter, RawExpression, Type},
    shared::ArithmethicOperator,
};

pub type StartState = Located<ParsingState>;

#[derive(Debug)]
pub enum ParsingState {
    Delimiter(ParsingDelimiter),
    Node(ParsingNode),
}
pub trait IntoParsingState {
    fn into_state(self) -> ParsingState;
}

#[derive(Debug)]
pub enum ParsingDelimiter {
    Function {
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
    },
    StartBlock,
    OpenParen,
    Return,
}
impl IntoParsingState for ParsingDelimiter {
    fn into_state(self) -> ParsingState {
        ParsingState::Delimiter(self)
    }
}

#[derive(Debug)]
pub enum ParsingNode {
    Expression(RawExpression),
    ArithmeticOperator(ArithmethicOperator),
}
impl IntoParsingState for ParsingNode {
    fn into_state(self) -> ParsingState {
        ParsingState::Node(self)
    }
}
