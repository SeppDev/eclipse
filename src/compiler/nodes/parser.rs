use std::fmt::Display;

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
    Expression(Expression),
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

impl ParserState {
    pub fn to_string_vec(vector: &Vec<Located<ParserState>>) -> String {
        Self::_to_string_vec(vector, 0)
    }
    fn _to_string_vec(vector: &Vec<Located<ParserState>>, indent: usize) -> String {
        vector
            .iter()
            .map(|s| s.raw._to_string(indent + 1))
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn _to_string(&self, indent: usize) -> String {
        let spacing = " ".repeat(indent);

        let raw = match &self {
            ParserState::Function {
                name,
                parameters,
                return_type,
                body,
            } => format!(
                "{}(): {} {{\n{}\n{spacing}}}",
                name.raw,
                match return_type {
                    Some(s) => s.raw.to_string(),
                    None => "void".to_string(),
                },
                Self::_to_string_vec(body, indent + 1)
            ),
            ParserState::Block(block) => {
                return format!(
                    "{spacing}{{\n {} \n{spacing}}}",
                    Self::_to_string_vec(block, indent + 1)
                )
            }
            ParserState::VarDecl {
                mutable,
                name,
                data_type,
                value,
            } => format!(
                "{} = {}",
                name.raw,
                value
                    .iter()
                    .map(|s| s.raw._to_string(0))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            ParserState::Identifier(string)
            | ParserState::Float(string)
            | ParserState::Integer(string) => string.clone(),
            ParserState::Return(expression) => {
                format!("return{}", Self::to_string_vec(expression))
            }
            ParserState::Expression(expression) => expression
                .iter()
                .map(|s| s.raw._to_string(0))
                .collect::<Vec<String>>()
                .join(" "),
            ParserState::ArithmeticOperator(operator) => {
                return match operator {
                    ArithmethicOperator::Plus => "+",
                    ArithmethicOperator::Minus => "-",
                    ArithmethicOperator::Multiply => "*",
                    ArithmethicOperator::Division => "/",
                    ArithmethicOperator::Remainder => "%",
                }
                .to_string()
            }
            ParserState::Operator(operator) => {
                return match operator {
                    Operator::Not => "!",
                }
                .to_string()
            }
        };

        format!("{spacing}{raw}")
    }
}

impl Located<ParserState> {
    pub fn to_string_vec(vector: &Vec<Located<ParserState>>) -> String {
        ParserState::_to_string_vec(vector, 0)
    }
}

impl Display for Located<ParserState> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}
impl Display for ParserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
