use std::{iter::Peekable, path::PathBuf, vec::IntoIter};

// use crate::{BuildError, BuildProblem, CompileError};

use crate::{
    parser::{ASTNode, Node},
    BuildError, CompileError,
};

use super::{Token, TokenInfo};

#[derive(Debug)]
pub struct TokensGroup {
    pub relative_path: PathBuf,

    current: TokenInfo,
    start: TokenInfo,

    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl TokensGroup {
    pub fn new(tokens: Vec<TokenInfo>, relative_path: PathBuf) -> Self {
        let start = tokens.first().unwrap().clone();
        let current = start.clone();

        let peekable: Peekable<IntoIter<TokenInfo>> = tokens.into_iter().peekable();

        return Self {
            start: start,
            current: current,
            relative_path: relative_path,
            tokens: peekable,
        };
    }
    pub fn create_error(&self, message: String) -> BuildError {
        let start = self.start.line;
        let end = self.current.line;
        return BuildError::CompileError(CompileError::new(message, start..end));
    }
    pub fn generate(&mut self, node: Node) -> Result<ASTNode, BuildError> {
        let start = self.start.line;
        let end = self.current.line;
        let ast = ASTNode {
            lines: start..end,
            node: node,
        };

        let info = self.peek()?;
        self.start = info;

        return Ok(ast);
    }
    pub fn peek(&mut self) -> Result<TokenInfo, BuildError> {
        return match self.tokens.peek() {
            Some(token) => Ok(token.to_owned()),
            None => return Err(self.create_error(format!("Early EndOfFile"))),
        };
    }
    pub fn advance(&mut self) -> Result<TokenInfo, BuildError> {
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
            None => return Err(self.create_error(format!("Early EndOfFile"))),
        };
    }
}
