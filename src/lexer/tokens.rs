use std::{iter::Peekable, slice::Iter};

use super::token::TokenInfo;


#[derive(Debug)]
pub struct Tokens<'a> {
    tokens: &'a mut Peekable<Iter<'a, TokenInfo>>,
}
impl<'a> Tokens<'a> {
    pub fn new(tokens: &'a mut Peekable<Iter<'a, TokenInfo>>) -> Self {
        return Self { tokens };
    }
    pub fn peek(&mut self) -> Option<TokenInfo> {
        return match self.tokens.peek() {
            Some(token) => Some(token.to_owned().to_owned()),
            None => None,
        };
    }
    pub fn next_token(&mut self) -> Option<TokenInfo> {
        return match self.tokens.next() {
            Some(token) => Some(token.to_owned()),
            None => None,
        };
    }
}
