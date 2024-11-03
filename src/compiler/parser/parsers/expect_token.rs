// pub fn parse_identifier() {

// }

use super::super::super::lexer::{Token, Tokens};
impl Tokens {
    pub fn expect_token(&mut self, expected: Token) {
        let info = self.advance();

        if info.token == expected {
            return;
        }

        self.throw_error(format!("Expected '{}', found '{}'", expected, info.token), "")
    }
    pub fn peek_expect_token(&mut self, token: Token) -> bool {
        return self.peek().token == token;
    }
}
