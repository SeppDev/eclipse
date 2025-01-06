use crate::compiler::{errors::CompileResult, lexer::Tokens, parser::Node};

impl Tokens {
    pub fn parse_namespace(&mut self) -> CompileResult<Node> {
        // let path = self.parse_path()?;
        // return Ok(self.create_located(RawNode::Use(path)));
        todo!()
    }
}
