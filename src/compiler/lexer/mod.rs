pub(super) mod kind;
pub(super) mod reader;
pub mod token;

use super::context::CompileCtx;
use crate::common::{errors::CompileResult, located::Located};
use kind::TokenKind;
use reader::Character;
use std::{ops::Range, path::PathBuf};
use token::{Token, TokenInfo};

impl CompileCtx {
    pub fn tokenize(&mut self, path: &PathBuf) -> CompileResult<Vec<TokenInfo>> {
        self.status
            .message(format!("Lexer: {:?}", path.clone().into_os_string()));

        let file = self.project_files.read(path)?.unwrap();
        let mut reader = self.new_reader(&file.body)?;

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
                TokenKind::Identifier(located) => {
                    TokenInfo::new(Token::Identifier(located.raw), located.position)
                }
                TokenKind::Integer(located) => {
                    TokenInfo::new(Token::Integer(located.raw), located.position)
                }
                TokenKind::Operators(chars) => {
                    loop {
                        let slice = chars.get()
                    }
                    continue;
                }
            };
            tokens.push(token);
        }

        Ok(tokens)
    }
}

fn chars_to_string(chars: Vec<Located<Character>>, range: Range<usize>) {}
