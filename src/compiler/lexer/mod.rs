pub(super) mod kind;
pub(super) mod reader;
pub mod token;

use crate::{
    common::position::{Position, PositionRange},
    compiler::diagnostics::DiagnosticResult,
};
use kind::{LexerKind, LocatedString};
use reader::{Character, LexerReader};
use std::ops::Range;
use token::{match_token, TokenInfo, TokenKind, MAX_OPERATOR_WIDTH};

pub fn tokenize(source: &str) -> DiagnosticResult<Vec<TokenInfo>> {
    let mut reader = LexerReader::new(source);
    let mut tokens = Vec::new();

    loop {
        let kind = match reader.next()? {
            Some(k) => k,
            None => break,
        };
        let token = match kind {
            LexerKind::String(position) => {
                TokenInfo::new(position.raw, TokenKind::String, position.position)
            }
            LexerKind::Character(position) => {
                TokenInfo::new(position.raw, TokenKind::Character, position.position)
            }
            LexerKind::Identifier(position) => {
                let kind = if let Some(kind) = match_token(&position.raw) {
                    kind
                } else {
                    TokenKind::Identifier
                };
                TokenInfo::new(position.raw, kind, position.position)
            }
            LexerKind::Integer(position) => {
                TokenInfo::new(position.raw, TokenKind::Integer, position.position)
            }
            LexerKind::Float(position) => {
                TokenInfo::new(position.raw, TokenKind::Float, position.position)
            }
            LexerKind::Operators(mut chars) => {
                let mut unkown = false;
                while chars.len() > 0 {
                    if unkown {
                        let char = chars.pop().unwrap();
                        tokens.push(TokenInfo::new(
                            char.raw.into(),
                            TokenKind::Unkown,
                            char.position,
                        ));
                        continue;
                    }

                    let len = chars.len().min(MAX_OPERATOR_WIDTH);

                    let mut is_final = true;
                    for i in 0..len {
                        let range = 0..len - i;
                        let string = chars_to_string(&chars, range.clone());

                        let kind = match match_token(&string.raw) {
                            Some(t) => t,
                            None => continue,
                        };

                        chars.drain(range);
                        tokens.push(TokenInfo::new(string.raw, kind, string.position));
                        is_final = false;
                        break;
                    }
                    unkown = is_final;
                }
                continue;
            }
        };
        tokens.push(token);
    }

    tokens.push(TokenInfo::new(
        "".into(),
        TokenKind::EndOfFile,
        Position::new(0, 0, 0).to_range(),
    ));
    Ok(tokens)
}

fn chars_to_string(chars: &Vec<Character>, range: Range<usize>) -> LocatedString {
    let slice = chars.get(range).unwrap();
    let body = slice.iter().map(|c| c.raw).collect::<String>();
    let start = slice.first().unwrap();
    let end = slice.last().unwrap();

    LocatedString::new(body, PositionRange::from(start.position, end.position))
}
