pub(super) mod kind;
pub(super) mod reader;
pub mod token;

use crate::{
    common::position::{Position, PositionRange},
    diagnostics::{DiagnosticData, DiagnosticResult},
};
use kind::{LexerKind, LocatedString};
use reader::Character;
use std::{ops::Range, path::PathBuf};
use token::{match_token, TokenInfo, TokenKind, MAX_OPERATOR_WIDTH};

use super::CompilerCtx;

impl CompilerCtx {
    pub fn tokenize(&mut self, relative_path: &PathBuf) -> DiagnosticResult<Vec<TokenInfo>> {
        self.message(format!("Lexer: {relative_path:?}"));

        let source = self.read_relative(relative_path)?;
        let mut reader = self.new_reader(source)?;

        let mut tokens = Vec::new();

        loop {
            let kind = match reader.next()? {
                Some(k) => k,
                None => break,
            };
            let token = match kind {
                LexerKind::String(located) => {
                    TokenInfo::new(located.raw, TokenKind::String, located.position)
                }
                LexerKind::Character(located) => {
                    TokenInfo::new(located.raw, TokenKind::Character, located.position)
                }
                LexerKind::Identifier(located) => {
                    let kind = if let Some(kind) = match_token(&located.raw) {
                        kind
                    } else {
                        TokenKind::Identifier
                    };
                    TokenInfo::new(located.raw, kind, located.position)
                }
                LexerKind::Integer(located) => {
                    TokenInfo::new(located.raw, TokenKind::Integer, located.position)
                }
                LexerKind::Float(located) => {
                    TokenInfo::new(located.raw, TokenKind::Float, located.position)
                }
                LexerKind::Operators(mut chars) => {
                    let mut unkown = false;
                    while chars.len() > 0 {
                        if unkown {
                            return Err(DiagnosticData::new(
                                format!("Unkown character: {:?}", chars.first().unwrap().raw),
                                relative_path.clone(),
                                "",
                                chars.first().unwrap().position,
                            ));
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
}

fn chars_to_string(chars: &Vec<Character>, range: Range<usize>) -> LocatedString {
    let slice = chars.get(range).unwrap();
    let body = slice.iter().map(|c| c.raw).collect::<String>();
    let start = slice.first().unwrap();
    let end = slice.last().unwrap();

    LocatedString::new(body, PositionRange::from(start.position, end.position))
}
