use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::Expression
};

impl Tokens {
    pub fn parse_arguments(&mut self) -> CompileResult<Vec<Expression>> {
        let mut arguments = Vec::new();

        loop {
            if self
                .peek_expect_tokens(vec![Token::CloseParen], true)
                .is_some()
            {
                break;
            };
            let expression = match self.parse_expression(true)? {
                Some(e) => e,
                None => break,
            };

            arguments.push(expression);
            let result = self.expect_tokens(vec![Token::Comma, Token::CloseParen], false)?;
            match result.token {
                Token::Comma => continue,
                Token::CloseParen => break,
                _ => panic!(),
            }
        }

        return Ok(arguments);
    }
}
