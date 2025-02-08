use crate::common::located::Located;

use super::reader::Character;

pub type LocatedString = Located<String>;

#[derive(Debug)]
pub enum TokenKind {
    String(LocatedString),
    Identifier(LocatedString),
    Integer(LocatedString),
    Operators(Vec<Character>),
}
