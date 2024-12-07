use crate::compiler::{errors::CompileResult, lexer::TokenInfo};

use super::super::super::lexer::{Token, Tokens};

impl Tokens {
    pub fn expect_tokens(&mut self, expected: Vec<Token>, start: bool) -> CompileResult<TokenInfo> {
        let info = if start {
            self.start()?
        } else {
            self.advance()?
        };

        for token in &expected {
            if info.token.better_eq(&token) {
                return Ok(info);
            }
        }

        self.error(
            info.location.clone(),
            format!(
                "Expected one of {}, found '{}'",
                expected
                    .into_iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<String>>()
                    .join(" or "),
                info.token
            ),
        );

        return Err(());
    }
    pub fn peek_require_tokens(&mut self, expected: Vec<Token>) -> CompileResult<TokenInfo> {
        let info = self.peek().clone();

        for token in &expected {
            if info.token.better_eq(&token) {
                return Ok(info);
            }
        }

        self.error(
            info.location.clone(),
            format!(
                "Expected one of {}, found '{}'",
                expected
                    .into_iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<String>>()
                    .join(" or "),
                info.token
            ),
        );

        return Ok(info);
    }
    pub fn peek_expect_tokens(
        &mut self,
        expected: Vec<Token>,
        advance_if_found: bool,
    ) -> Option<TokenInfo> {
        let info = self.peek().clone();

        for token in &expected {
            if token.better_eq(&info.token) {
                if advance_if_found {
                    let _ = self.advance();
                }
                return Some(info);
            }
        }

        return None;
    }
}
