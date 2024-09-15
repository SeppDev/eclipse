use std::{iter::Peekable, path::PathBuf, vec::IntoIter};

// use crate::{BuildError, BuildProblem, CompileError};

use crate::CompileError;

use super::{Token, TokenInfo};

#[derive(Debug)]
pub struct TokensGroup {
    pub current: TokenInfo,
    pub relative_path: PathBuf,
    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl TokensGroup {
    pub fn new(tokens: Vec<TokenInfo>, relative_path: PathBuf) -> Self {
        let peekable = tokens.into_iter().peekable();
        return Self {
            current: TokenInfo::default(),
            relative_path: relative_path,
            tokens: peekable,
        };
    }
    pub fn peek(&mut self) -> Result<TokenInfo, CompileError> {
        // println!("PEEK {:#?} PEEK", self.tokens);
        return match self.tokens.peek() {
            Some(token) => Ok(token.to_owned()),
            None => return Err(CompileError::BuildProblem),
        };
    }
    pub fn advance(&mut self) -> Result<TokenInfo, CompileError> {
        // println!("NEXT {:#?} NEXT", self.tokens);
        match self.current.token {
            Token::EndOfFile => {
                panic!("Failed to handle EndOFile")
            }
            _ => {}
        }

        return match self.tokens.next() {
            Some(info) => {
                self.current = info.clone();
                Ok(info)
            }
            None => return Err(CompileError::BuildProblem),
        };
    }
}
