use crate::compiler::lexer::TokenInfo;

use super::super::super::lexer::{Token, Tokens};

impl Tokens {
    pub fn expect_tokens(&mut self, expected: Vec<Token>, start: bool) -> TokenInfo {
        let info = if start { self.start() } else { self.advance() };

        for token in &expected {
            if info.token.better_eq(&token) {
                return info;
            }
        }

        self.throw_error(
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
        )
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
    pub fn peek_expect_token(&mut self, expected: Token, advance_if_found: bool) -> bool {
        let result = self.peek().token.better_eq(&expected);
        if result && advance_if_found {
            self.advance();
        }
        result
    }
}
