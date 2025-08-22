use common::position::Span;

use super::reader::Character;

pub type LocatedString = Span<String>;

#[derive(Debug)]
pub enum LexerKind {
    String(LocatedString),
    Character(LocatedString),
    Identifier(LocatedString),
    Integer(LocatedString),
    Float(LocatedString),
    Operators(Vec<Character>),
}
