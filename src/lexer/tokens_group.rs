use std::{iter::Peekable, vec::IntoIter};

use crate::{
    parser::{ASTNode, Node},
    CompileError, ParseResult,
};

use super::{Token, TokenInfo};

#[derive(Debug)]
pub struct TokensGroup {
    // pub relative_path: PathBuf,
    pub current: TokenInfo,
    pub indent: usize,

    starts: Vec<TokenInfo>,
    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl TokensGroup {
    pub fn new(tokens: Vec<TokenInfo> /*relative_path: PathBuf*/) -> Self {
        let start = tokens.first().unwrap().clone();
        let current = start.clone();

        let peekable: Peekable<IntoIter<TokenInfo>> = tokens.into_iter().peekable();

        return Self {
            indent: 0,
            starts: Vec::new(),
            current,
            tokens: peekable,
        };
    }
    pub fn create_error(&self, message: String) -> CompileError {
        let start = self.current.line;
        return CompileError::new(message, start);
    }
    pub fn create_ast(&mut self, node: Node) -> ASTNode {
        let start = self.starts.pop().unwrap();
        return ASTNode::new(self.indent, start.line..self.current.line, node);
    }
    pub fn peek(&mut self) -> ParseResult<TokenInfo> {
        return match self.tokens.peek() {
            Some(token) => Ok(token.to_owned()),
            None => return Err(self.create_error(format!("Early EndOfFile"))),
        };
    }
    pub fn is_eof(&mut self) -> ParseResult<bool> {
        return Ok(match self.peek()?.token {
            Token::EndOfFile => true,
            _ => false,
        });
    }
    pub fn start(&mut self) -> ParseResult<TokenInfo> {
        let info = self.advance()?;
        self.starts.push(info.clone());
        return Ok(info);
    }
    pub fn advance(&mut self) -> ParseResult<TokenInfo> {
        return match self.tokens.next() {
            Some(info) => {
                self.current = info.clone();

                match info.token {
                    Token::StartScope => self.indent += 1,
                    Token::EndScope => self.indent -= 1,
                    _ => {}
                }
                Ok(info)
            }
            None => return Err(self.create_error(format!("Early EndOfFile"))),
        };
    }
}
