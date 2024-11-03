// pub fn parse_identifier() {

// }

use super::super::super::lexer::{Token, Tokens};
impl Tokens {
    pub fn parse_identifer(&mut self) -> String {
        let info = self.advance();

        let token = match &info.token {
            Token::Identifier(string) => return string.clone(),
            token => token.clone(),
        };
        
        self.throw_error(
            format!("Expected identifier, found '{:?}'", token),
            "expected identifier",
        )
    }
}
