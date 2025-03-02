use std::borrow::BorrowMut;

use crate::{common::position::Located, compiler::lexer::token::TokenInfo};

use super::{
    ast::{Identifier, Node, Parameter, Type},
    shared::ArithmethicOperator,
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
    VarDecl {
        mutable: Option<TokenInfo>,
        name: Identifier,
        data_type: Option<Type>,
        value: Expression,
    },
    Block(Expression),
    Return(Expression),
    Expression(Expression),
    Integer(String),
    Float(String),
    Identifier(String),
    ArithmeticOperator(ArithmethicOperator),
}

impl ParserState {
    pub fn is_block(&self) -> bool {
        match self {
            ParserState::Block(..) | ParserState::Function { .. } => true,
            _ => false,
        }
    }
    pub fn is_expression(&self) -> bool {
        match self {
            ParserState::Identifier(..) | ParserState::Integer(..) => true,
            _ => false,
        }
    }
    pub fn block(&mut self) -> Option<&mut Expression> {
        Some(match self {
            ParserState::Block(block) => block,
            ParserState::Function { body, .. } => body,
            _ => return None,
        })
    }
    pub fn expression_body(&mut self) -> Option<&mut Expression> {
        Some(match self {
            ParserState::Return(value) | ParserState::VarDecl { value, .. } => value,
            _ => return None,
        })
    }
}
