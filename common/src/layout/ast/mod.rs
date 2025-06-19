use crate::{lexer::token::Token, position::LocatedAt};

mod node;
mod types;

pub use node::*;
pub use types::*;

pub type Node = LocatedAt<RawNode>;

pub type Location = LocatedAt<()>;
pub type Type = LocatedAt<RawType>;
pub type Parameter = LocatedAt<RawParameter>;
pub type Modifier = LocatedAt<RawModifier>;

pub type Identifier = LocatedAt<String>;
impl From<Token> for Identifier {
    fn from(value: Token) -> Self {
        LocatedAt {
            position: value.position,
            raw: value.string,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RawParameter {
    pub reference: Option<Token>,
    pub mutable: Option<Token>,
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
