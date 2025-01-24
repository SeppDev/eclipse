use crate::compiler::{
    errors::{CompileCtx, CompileMessage, CompileResult},
    nodes::ast::Located,
    path::Path,
};

use crate::common::location::PositionRange;

use super::{Token, TokenInfo};
use std::{fmt::Debug, iter::Peekable, vec::IntoIter};

#[derive(Debug)]
pub struct Tokens {
    pub relative_file_path: Path,

    start_on_next: bool,
    messages: Vec<CompileMessage>,
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

    pub fn error<T: ToString>(
        &mut self,
        position: PositionRange,
        message: T,
    ) -> &mut CompileMessage {
        let mut message = CompileMessage::error(message.to_string());
        message.push("", position);
        self.messages.push(message);
        return self.messages.last_mut().unwrap();
    }

    pub fn finish(self, ctx: &mut CompileCtx) {
        for message in self.messages {
            ctx.push(self.relative_file_path.clone(), message);
        }
        for message in self.result_messages {
            ctx.result_print(message)
        }
        if self.starts.len() > 0 {
            ctx.result_print(format!(
                "Failed to use all the start nodes! {:#?}",
                self.starts
            ));
        }
    }
    pub fn current(&self) -> &TokenInfo {
        return self.current.as_ref().unwrap();
    }
    pub fn pop_start(&mut self) -> TokenInfo {
        self.starts.pop().unwrap()
    }
    pub fn create_located<T: Debug>(&mut self, raw: T) -> Located<T> {
        let start = self
            .starts
            .pop()
            .unwrap_or_else(|| panic!("Missing starting node for: {raw:#?}"));

        let current = self.current.clone().unwrap().position;
        let position = start.position.start.extend(current.end);
        Located::new(position, raw)
    }
    pub fn start(&mut self) -> CompileResult<TokenInfo> {
        let token = self.advance()?;
        self.starts.push(token.clone());
        Ok(token)
    }
    pub fn start_current(&mut self) -> CompileResult<TokenInfo> {
        let token = self.current().clone();
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
                    current.position.clone(),
                    format!("No token found {}", current.token),
                );
                return Err(());
            }
        };

        match info.token {
            Token::EndOfFile => {
                self.error(info.position.clone(), format!("Early {}", info.token));
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
