use crate::compiler::{errors::MessageKind, lexer::TokenInfo};

use super::super::super::lexer::{Token, Tokens};

impl Tokens {
    pub fn expect_tokens(&mut self, expected: Vec<Token>, start: bool) -> TokenInfo {
        let info = if start { self.start() } else { self.advance() };

        for token in &expected {
            if info.token.better_eq(&token) {
                return info;
            }
        }

        self.throw(
            MessageKind::Error,
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
            "",
        );
        info
    }
    pub fn peek_require_token(&mut self, expected: Vec<Token>) -> TokenInfo {
        let info = self.peek().clone();

        for token in &expected {
            if info.token.better_eq(&token) {
                return info;
            }
        }

        self.throw(
            MessageKind::Error,
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
            "",
        );

        return info;
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
                    self.advance();
                }
                return Some(info);
            }
        }

        return None;
    }
}
