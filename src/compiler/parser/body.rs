use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Identifier, Node, RawNode}
};

impl Tokens {
    pub fn parse_body(&mut self) -> CompileResult<Vec<Node>> {
        let mut body: Vec<Node> = Vec::new();

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
                    self.create_located(RawNode::Scope(nodes))
                }
                Token::Return => {
                    let expression = self.parse_expression(false)?;
                    self.create_located(RawNode::Return(expression))
                }
                Token::Result => {
                    let expression = self.parse_expression(false)?;
                    self.create_located(RawNode::Result(expression))
                }
                Token::Continue => self.create_located(RawNode::Continue),
                Token::Break => self.create_located(RawNode::Break),
                Token::If => self.parse_ifstatement()?,
                Token::Use => self.parse_namespace()?,
                Token::Identifier(name) => {
                    let identifier = Identifier {
                        location: info.location,
                        raw: name,
                    };
                    
                    let path = self.parse_path_current(identifier.raw)?;
                    self.parse_after_path(path)?
                },
                Token::Loop => self.parse_loop()?,
                Token::While => self.parse_while()?,
                Token::Variable => self.parse_variable()?,
                _ => continue,
            };
            body.push(node)
        }

        return Ok(body);
    }
}
