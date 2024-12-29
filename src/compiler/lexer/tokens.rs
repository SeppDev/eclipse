use crate::compiler::{
    errors::{CompileCtx, CompileResult, Location, Message},
    parser::{Expression, ExpressionInfo, Node, NodeInfo},
    path::Path
};

use super::{Token, TokenInfo};
use std::{iter::Peekable, vec::IntoIter};

#[derive(Debug)]
pub struct Tokens {
    pub relative_file_path: Path,
    
    start_on_next: bool,
    messages: Vec<Message>,
    result_messages: Vec<String>,
    current: Option<TokenInfo>,
    starts: Vec<TokenInfo>,
    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl Tokens {
    pub fn new(tokens: Vec<TokenInfo>, relative_file_path: Path) -> Self {
        return Self {
            relative_file_path,
            start_on_next: false,
            result_messages: Vec::new(),
            messages: Vec::new(),
            starts: Vec::new(),
            current: None,
            tokens: tokens.into_iter().peekable(),
        };
    }

    pub fn error<T: ToString>(&mut self, location: Location, message: T) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.messages.push(message);
        return self.messages.last_mut().unwrap();
    }
    pub fn result_print(&mut self, message: String) {
        self.result_messages.push(message);
    }

    pub fn finish(self, debug: &mut CompileCtx) {
        for message in self.messages {
            debug.push(self.relative_file_path.clone(), message);
        }
        for message in self.result_messages {
            debug.result_print(message)
        }
        if self.starts.len() > 0 {
            debug.result_print("Failed to use all the start nodes!");
        }
    }
    pub fn current(&self) -> &TokenInfo {
        return self.current.as_ref().unwrap();
    }
    pub fn pop_start(&mut self) -> TokenInfo {
        self.starts.pop().unwrap()
    }
    pub fn create_node(&mut self, node: Node) -> NodeInfo {
        let start = self.starts.pop().unwrap_or_else(|| {
            panic!("No starting node for: {:#?}", node);
        });
        let current = self.current.clone().unwrap().location;
        let location = Location::new(
            start.location.lines.start..current.lines.end,
            start.location.columns.start..current.columns.end,
        );
        NodeInfo { node, location }
    }
    pub fn create_expression(&mut self, expression: Expression) -> ExpressionInfo {
        let start = self.starts.pop().unwrap_or_else(|| {
            panic!("No starting node for: {:#?}", expression);
        });
        let current = self.current.clone().unwrap().location;
        let location = Location::new(
            start.location.lines.start..current.lines.end,
            start.location.columns.start..current.columns.end,
        );

        ExpressionInfo {
            expression,
            location,
        }
    }
    pub fn start(&mut self) -> CompileResult<TokenInfo> {
        let token = self.advance()?; 
        self.starts.push(token.clone());
        Ok(token)
    }
    pub fn start_next(&mut self) {
        self.start_on_next = true;
    }
    pub fn advance(&mut self) -> CompileResult<TokenInfo> {
        let info = if self.start_on_next {
            self.start_on_next = false;
            return self.start();
        } else {
            self.tokens.next()
        };

        let info = match info {
            Some(info) => info,
            None => {
                let current = self.current.as_ref().unwrap();
                self.error(
                    current.location.clone(),
                    format!("No token found {}", current.token),
                );
                return Err(());
            }
        };

        match info.token {
            Token::EndOfFile => {
                self.error(info.location.clone(), format!("Early {}", info.token));
            }

            _ => {}
        }
        self.current = Some(info.clone());
        return Ok(info);
    }
    pub fn peek(&mut self) -> &TokenInfo {
        match self.tokens.peek() {
            Some(info) => return info,
            None => {}
        };
        self.current.as_mut().unwrap()
    }
    pub fn is_eof(&mut self) -> bool {
        match self.tokens.peek() {
            Some(info) => info.token == Token::EndOfFile,
            None => true,
        }
    }
}
