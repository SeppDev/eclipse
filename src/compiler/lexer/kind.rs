use crate::common::position::PositionRange;

use super::reader::Character;

pub enum TokenKind {
    String(PositionRange, String),
    Identifier(PositionRange, String),
    Integer(PositionRange, String),
    Float(PositionRange, String, String),
    Punctuation(Character),
}
