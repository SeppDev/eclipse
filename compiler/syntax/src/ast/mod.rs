mod module;
mod node;
mod types;

pub use module::*;
pub use node::*;
pub use types::*;

use common::position::Span;

pub type Node = Span<RawNode>;

pub type Location = Span<()>;
pub type Type = Span<types::RawType>;
pub type Parameter = Span<RawParameter>;
pub type Modifier = Span<RawModifier>;
pub type Attribute = Span<RawAttribute>;

pub type Identifier = Span<String>;

#[derive(Debug, PartialEq, Clone)]
pub struct RawParameter {
    pub reference: Option<Span>,
    pub mutable: Option<Span>,
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
