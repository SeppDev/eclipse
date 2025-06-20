mod node;
mod types;

pub use node::*;
pub use types::*;

use common::position::LocatedAt;

pub type Node = LocatedAt<RawNode>;

pub type Location = LocatedAt<()>;
pub type Type = LocatedAt<RawType>;
pub type Parameter = LocatedAt<RawParameter>;
pub type Modifier = LocatedAt<RawModifier>;

pub type Identifier = LocatedAt<String>;

#[derive(Debug, PartialEq, Clone)]
pub struct RawParameter {
    pub reference: Option<LocatedAt>,
    pub mutable: Option<LocatedAt>,
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
