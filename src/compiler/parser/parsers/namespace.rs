use crate::compiler::{
    errors::CompileResult,
    lexer::Tokens,
    parser::{Node, NodeInfo},
};

impl Tokens {
    pub fn parse_namespace(&mut self, public: bool) -> CompileResult<NodeInfo> {
        let root = self.parse_identifier()?;
        let path = self.parse_path(&root)?;
        return Ok(self.create_node(Node::NameSpace {
            public,
            static_path: path,
        }));
    }
}
