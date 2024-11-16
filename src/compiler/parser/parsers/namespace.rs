use crate::compiler::{
    errors::CompileResult,
    lexer::Tokens,
    parser::{Node, NodeInfo},
};

use super::path::parse_path;

pub fn parse_namespace(tokens: &mut Tokens, public: bool) -> CompileResult<NodeInfo> {
    let root = tokens.parse_identifier()?;
    let path = parse_path(tokens, &root)?;
    return Ok(tokens.create_node(Node::NameSpace {
        public,
        static_path: path,
    }));
}
