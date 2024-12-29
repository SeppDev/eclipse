use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

impl Tokens {
    pub fn parse_body(&mut self) -> CompileResult<Vec<NodeInfo>> {
        let mut body: Vec<NodeInfo> = Vec::new();

        loop {
            while self
                .peek_expect_tokens(vec![Token::SemiColon], true)
                .is_some()
            {}
            
            if self
                .peek_expect_tokens(vec![Token::EndScope], true)
                .is_some()
            {
                break;
            }


            let info = self.expect_tokens(
                vec![
                    Token::Return,
                    Token::Function,
                    Token::Variable,
                    Token::StartScope,
                    Token::If,
                    Token::Use,
                    Token::Loop,
                    Token::While,
                    Token::Continue,
                    Token::Break,
                    Token::Identifier(String::new()),
                ],
                true,
            )?;

            let node = match info.token {
                Token::StartScope => {
                    let nodes = self.parse_body()?;
                    self.create_node(Node::Scope(nodes))
                }
                Token::Continue => self.create_node(Node::Continue),
                Token::Break => self.create_node(Node::Break),
                Token::If => self.parse_ifstatement()?,
                Token::Use => self.parse_namespace(false)?,
                Token::Identifier(name) => self.parse_after_identifier(name)?,
                Token::Loop => self.parse_loop()?,
                Token::While => self.parse_while()?,
                Token::Return => {
                    let expression = self.parse_expression(false)?;
                    self.create_node(Node::Return(expression))
                }
                Token::Variable => self.parse_variable()?,
                _ => continue,
            };
            body.push(node)
        }

        return Ok(body);
    }
}
