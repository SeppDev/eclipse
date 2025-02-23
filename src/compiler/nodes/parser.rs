use crate::compiler::lexer::token::TokenInfo;

use super::{
    ast::{Identifier, Node, Parameter, RawNode, Type},
    shared::ArithmethicOperator,
};

#[derive(Debug)]
pub enum ParserState {
    Function {
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Node>,
    },
    VarDecl {
        mutable: Option<TokenInfo>,
        name: Identifier,
        data_type: Option<Type>,
    },
    Block(Vec<Node>),
    Return,
    Expression(RawNode),
    ArithmeticOperator(ArithmethicOperator),
}
