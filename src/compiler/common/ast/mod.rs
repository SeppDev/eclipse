mod node;
mod types;

pub use node::*;
pub use types::*;

use super::operators::{ArithmethicOperator, CompareOperator, EqualsOperation};
use crate::{common::position::LocatedAt, compiler::lexer::token::TokenInfo};

pub type Node = LocatedAt<RawNode>;

pub type Location = LocatedAt<()>;
pub type Type = LocatedAt<RawType>;
pub type Parameter = LocatedAt<RawParameter>;
pub type Modifier = LocatedAt<RawModifier>;

pub type Identifier = LocatedAt<String>;
impl From<TokenInfo> for Identifier {
    fn from(value: TokenInfo) -> Self {
        LocatedAt {
            position: value.position,
            raw: value.string,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RawParameter {
    pub reference: Option<TokenInfo>,
    pub mutable: Option<TokenInfo>,
    pub name: Identifier,
    pub data_type: Type,
}

#[derive(Debug, PartialEq)]
pub enum UsePath {
    Ident(Identifier),
    Extend(Identifier, Box<UsePath>),
    List(Vec<UsePath>),
}

#[derive(Debug, PartialEq)]
pub enum RawModifier {
    Pub,
    Static,
    Async,
    Unsafe,
    Extern(Identifier),
}
