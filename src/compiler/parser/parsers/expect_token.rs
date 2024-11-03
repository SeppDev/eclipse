// pub fn parse_identifier() {

// }

use crate::compiler::lexer::TokenInfo;

use super::super::super::lexer::{Token, Tokens};
impl Tokens {
    pub fn expect_token(&mut self, expected: Token) {
        let info = self.advance();

        if info.token == expected {
            return;
        }

        self.throw_error(
            format!("Expected '{}', found '{}'", expected, info.token),
            "",
        )
    }
    pub fn expect_tokens(&mut self, expected: Vec<Token>) -> TokenInfo {
        let info = self.advance();
        
        for token in expected {
            if token == info.token {
                return info
            }
        }
        
        self.throw_error(
            format!("Expected '{}', found '{}'", expected.join(", "), info.token),
            "",
        )
    }
    pub fn peek_expect_token(&mut self, expected: Token) -> bool {
        return self.peek().token == expected;
    }
}
