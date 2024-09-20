use std::{iter::Peekable, path::PathBuf, vec::IntoIter};

use crate::{
    parser::{ASTNode, Node}, CompileError, ParseResult
};

use super::{Token, TokenInfo};

#[derive(Debug)]
pub struct TokensGroup {
    pub relative_path: PathBuf,

    pub current: TokenInfo,
    pub start: TokenInfo,

    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl TokensGroup {
    pub fn new(tokens: Vec<TokenInfo>, relative_path: PathBuf) -> Self {
        let start = tokens.first().unwrap().clone();
        let current = start.clone();

        let peekable: Peekable<IntoIter<TokenInfo>> = tokens.into_iter().peekable();

        return Self {
            start,
            current,
            relative_path,
            tokens: peekable,
        };
    }
    pub fn create_error(&self, message: String) -> CompileError {
        let start = self.start.line;
        return CompileError::new(message, start);
    }
    pub fn create_AST(&self, node: Node) -> ASTNode {
        return ASTNode::new(self.start.line..self.current.line, node);
    }
    pub fn peek(&mut self) -> ParseResult<TokenInfo> {
        return match self.tokens.peek() {
            Some(token) => Ok(token.to_owned()),
            None => return Err(self.create_error(format!("Early EndOfFile"))),
        };
    }
    pub fn start(&mut self) -> ParseResult<TokenInfo> {
        let token = self.advance()?;
        self.start = token.clone();
        return Ok(token);
    }
    pub fn advance(&mut self) -> ParseResult<TokenInfo> {
        match self.current.token {
            Token::EndOfFile => return Err(CompileError::new(format!("Failed to handle EndOfFile"), self.current.line)),
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
