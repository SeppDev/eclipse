use crate::common::position::LocatedAt;

use super::reader::Character;

pub type LocatedString = LocatedAt<String>;

#[derive(Debug)]
pub enum LexerKind {
    String(LocatedString),
    Character(LocatedString),
    Identifier(LocatedString),
    Integer(LocatedString),
    Float(LocatedString),
    Operators(Vec<Character>),
}
