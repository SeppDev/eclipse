use crate::compiler::{errors::CompileResult, lexer::TokenInfo};

use super::super::lexer::{Token, Tokens};

impl Tokens {
    pub fn expect_tokens(
        &mut self,
        mut expected: Vec<Token>,
        start: bool,
    ) -> CompileResult<TokenInfo> {
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

        self.error(info.position, format_expected(&mut expected, &info.token));

        return Err(());
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

fn format_expected(expected: &mut Vec<Token>, got: &Token) -> String {
    let len = expected.len();
    let expected = if len <= 1 {
        format!("{}", expected.first().unwrap())
    } else {
        let last = format!(" or {}", expected.pop().unwrap());
        let mut body = expected
            .into_iter()
            .map(|x| format!("'{}'", x))
            .collect::<Vec<String>>()
            .join(", ");

        body.push_str(last.as_str());

        format!("one of {body}")
    };

    format!("Expected {expected}, found '{got}'")
}
