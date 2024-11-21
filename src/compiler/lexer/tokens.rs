use crate::compiler::{
    errors::{create_error_message, CompileMessages, Location, Message, MessageKind},
    parser::{Expression, ExpressionInfo, Node, NodeInfo},
    path::Path,
};

use super::{Token, TokenInfo};
use std::{iter::Peekable, vec::IntoIter};

#[derive(Debug)]
pub struct Tokens {
    relative_path: Path,
    messages: Vec<Message>,
    current: Option<TokenInfo>,
    starts: Vec<TokenInfo>,
    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl Tokens {
    pub fn new(tokens: Vec<TokenInfo>, relative_path: Path) -> Self {
        return Self {
            relative_path,
            messages: Vec::new(),
            starts: Vec::new(),
            current: None,
            tokens: tokens.into_iter().peekable(),
        };
    }
    pub fn throw<T: ToString, E: ToString>(
        &mut self,
        kind: MessageKind,
        location: Location,
        message: T,
        notice: E,
    ) -> &mut Message {
        let message = create_error_message(kind, location, message, notice);
        self.messages.push(message);
        return self.messages.last_mut().unwrap();
    }
    pub fn finish(self, compile_messages: &mut CompileMessages) {
        for message in self.messages {
            compile_messages.push(self.relative_path.clone(), message);
        }
    }
    pub fn pop_start(&mut self) -> TokenInfo {
        self.starts.pop().unwrap()
    }
    // pub fn push_start(&mut self, token: &TokenInfo) {
    //     self.starts.push(token.clone());
    // }
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
    pub fn start(&mut self) -> TokenInfo {
        let token = self.advance();
        self.starts.push(token.clone());
        token
    }
    pub fn advance(&mut self) -> TokenInfo {
        match self.tokens.next() {
            Some(info) => {
                match info.token {
                    Token::EndOfFile => {
                        self.throw(
                            MessageKind::Error,
                            info.location.clone(),
                            format!("Early {}", info.token),
                            "",
                        );
                    }
                    _ => {}
                }
                self.current = Some(info.clone());
                info
            }
            None => {
                let current = self.current.clone().unwrap();
                self.throw(
                    MessageKind::Error,
                    current.location.clone(),
                    format!("No token found {}", current.token),
                    "",
                );
                current
            }
        }
    }
    pub fn peek(&mut self) -> &TokenInfo {
        match self.tokens.peek() {
            Some(info) => return info,
            None => {}
        };
        self.current.as_mut().unwrap()
    }
    pub fn is_eof(&mut self) -> bool {
        return self.tokens.peek().unwrap().token == Token::EndOfFile;
    }
}
