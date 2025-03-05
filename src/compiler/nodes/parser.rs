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
    VarDecl {
        mutable: Option<TokenInfo>,
        name: Identifier,
        data_type: Option<Type>,
        value: Expression,
    },
    Block(Expression),
    Return(Expression),
    Integer(String),
    Float(String),
    Identifier(String),
    ArithmeticOperator(ArithmethicOperator),
    Operator(Operator),
}

impl ParserState {
    pub fn is_block(&self) -> bool {
        match self {
            ParserState::Block(..) | ParserState::Function { .. } => true,
            _ => false,
        }
    }
    pub fn block(&mut self) -> Option<&mut Expression> {
        Some(match self {
            ParserState::Function { body, .. } | ParserState::Block(body) => body,
            _ => return None,
        })
    }

    pub fn node_body(&mut self) -> Option<&mut Expression> {
        Some(match self {
            ParserState::Return(value) | ParserState::VarDecl { value, .. } => value,
            _ => return None,
        })
    }
    pub fn expects_expression(&self) -> bool {
        let expressions = match self {
            ParserState::Return(value) | ParserState::VarDecl { value, .. } => value,
            _ => return false,
        };

        let last = match expressions.last() {
            Some(l) => l,
            None => return true,
        };

        return last.raw.is_operator();
    }
    pub fn is_node(&self) -> bool {
        match self {
            ParserState::Return(..) | ParserState::VarDecl { .. } => true,
            _ => false,
        }
    }

    pub fn is_expression(&self) -> bool {
        match self {
            ParserState::Identifier(..)
            | ParserState::Integer(..)
            | ParserState::Float(..)
            | ParserState::Block(..) => true,
            _ => false,
        }
    }
    pub fn is_operator(&self) -> bool {
        match self {
            ParserState::Operator(..) | ParserState::ArithmeticOperator(..) => true,
            _ => false,
        }
    }
}
