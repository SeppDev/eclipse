use crate::{common::position::Located, compiler::lexer::token::TokenInfo};

use super::{
    ast::{Identifier, Parameter, Type},
    shared::{ArithmethicOperator, Operator},
};

type Expression = Vec<Located<ParserState>>;

#[derive(Debug)]
pub enum ParserState {
    Function {
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Expression,
    },
    OpenBlock {
        body: Expression,
    },
    Block {
        body: Expression,
    },
    Conditional {
        condition: Expression,
        body: Expression,
    },
    VarDecl {
        mutable: Option<TokenInfo>,
        name: Identifier,
        data_type: Option<Type>,
        value: Expression,
    },
    BodyLess(Expression),
    Return(Expression),
    Continue(Expression),
    Break(Expression),

    Integer(String),
    Float(String),
    Identifier(String),
    ArithmeticOperator(ArithmethicOperator),
    Operator(Operator),
}
