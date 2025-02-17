pub(super) mod kind;
pub(super) mod reader;
pub mod token;

use crate::{
    common::position::{Position, PositionRange},
    diagnostics::DiagnosticResult,
};
use kind::{LocatedString, TokenKind};
use reader::Character;
use std::{ops::Range, path::PathBuf};
use token::{match_token, Token, TokenInfo, MAX_OPERATOR_WIDTH};

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
                TokenKind::String(located) => {
                    TokenInfo::new(Token::String(located.raw), located.position)
                }
                TokenKind::Character(located) => {
                    TokenInfo::new(Token::Character(located.raw), located.position)
                }
                TokenKind::Identifier(located) => {
                    if let Some(token) = match_token(&located.raw) {
                        TokenInfo::new(token, located.position)
                    } else {
                        TokenInfo::new(Token::Identifier(located.raw), located.position)
                    }
                }
                TokenKind::Integer(located) => {
                    TokenInfo::new(Token::Integer(located.raw), located.position)
                }
                TokenKind::Float(located) => {
                    TokenInfo::new(Token::Float(located.raw), located.position)
                }
                TokenKind::Operators(mut chars) => {
                    while chars.len() > 0 {
                        let len = chars.len().min(MAX_OPERATOR_WIDTH);

                        for i in 0..len {
                            let range = 0..len - i;
                            let string = chars_to_string(&chars, range.clone());

                            let token = match match_token(&string.raw) {
                                Some(t) => t,
                                None => continue,
                            };
                            chars.drain(range);
                            tokens.push(TokenInfo::new(token, string.position));
                            break;
                        }
                    }
                    continue;
                }
            };
            tokens.push(token);
        }

        tokens.push(TokenInfo::new(
            Token::EndOfFile,
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
