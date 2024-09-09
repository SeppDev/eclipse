use std::{iter::Peekable, path::PathBuf, vec::IntoIter};

// use crate::{BuildError, BuildProblem, CompileError};

use crate::{BuildError, BuildProblem, CompileError};

use super::TokenInfo;

#[derive(Debug)]
pub struct TokensGroup {
    pub current: TokenInfo,
    pub relative_path: PathBuf,
    tokens: Peekable<IntoIter<TokenInfo>>,
    token_history: Vec<TokenInfo>,
}
impl TokensGroup {
    pub fn new(tokens: Vec<TokenInfo>, relative_path: PathBuf) -> Self {
        let peekable = tokens.into_iter().peekable();
        return Self {
            current: TokenInfo::default(),
            relative_path: relative_path,
            token_history: Vec::new(),
            tokens: peekable,
        };
    }
    pub fn peek(&mut self) -> Result<TokenInfo, CompileError> {
        // println!("PEEK {:#?} PEEK", self.tokens);
        return match self.tokens.peek() {
            Some(token) => Ok(token.to_owned()),
            None => {
                return Err(CompileError::BuildProblem(BuildProblem::new(
                    BuildError::Peekfail,
                    self.relative_path.clone(),
                    self.current.line,
                )))
            }
        };
    }
    // pub fn previous_tokens(&mut self, mut length: usize) -> Vec<TokenInfo> {
    //     let len = self.token_history.len();
    //     length += 1;
    //     self.token_history
    //         .get(len.max(length) - length..len - 1)
    //         .unwrap()
    //         .to_vec()
    // }
    pub fn advance(&mut self) -> Result<TokenInfo, CompileError> {
        // println!("NEXT {:#?} NEXT", self.tokens);
        return match self.tokens.next() {
            Some(info) => {
                self.token_history.push(info.clone());
                self.current = info.clone();
                Ok(info)
            }
            None => {
                return Err(CompileError::BuildProblem(BuildProblem::new(
                    BuildError::NoTokenFound,
                    self.relative_path.clone(),
                    self.token_history.get(0).unwrap().line,
                )))
            }
        };
    }
}
