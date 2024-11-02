use std::{iter::Peekable, vec::IntoIter};


use super::{Token, TokenInfo};

#[derive(Debug)]
pub struct TokensGroup {
    indent: usize,
    starts: Vec<&'static TokenInfo>,
    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl TokensGroup {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        let peekable: Peekable<IntoIter<TokenInfo>> = tokens.into_iter().peekable();

        return Self {
            indent: 0,
            starts: Vec::new(),
            tokens: peekable,
        };
    }
    // pub fn create_error(&self, message: String) -> CompileError {
    //     let start = self.current.line;
    //     return CompileError::new(message, start);
    // }
    // pub fn create_ast(&mut self, node: Node) -> ASTNode {
    //     let start = self.starts.pop().unwrap();
    //     return ASTNode::new(start.line..self.current.line, node);
    // }
    pub fn peek(&mut self) -> Option<&TokenInfo> {
        return match self.tokens.peek() {
            Some(token) => Some(token),
            None => return None,//Err(self.create_error(format!("Early EndOfFile"))),
        };
    }
    pub fn is_eof(&mut self) -> bool {
        return match self.peek() {
            Some(t) => match t.token {
                Token::EndOfFile => true,
                _ => false,
            },
            None => false,
        };
    }
    pub fn start(&mut self) -> Option<&TokenInfo> {
        let info = self.advance()?;
        self.starts.push(info);
        return Some(info);
    }
    pub fn advance(&mut self) -> Option<&TokenInfo> {
        return match self.tokens.next() {
            Some(info) => {
                match info.token {
                    Token::StartScope => self.indent += 1,
                    Token::EndScope => self.indent -= 1,
                    _ => {}
                }
                Some(info)
            }
            None => return None,// Err(self.create_error(format!("Early EndOfFile"))),
        };
    }
}
