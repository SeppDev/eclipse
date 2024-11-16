use crate::compiler::{
    lexer::Tokens,
    parser::{Node, NodeInfo},
};

use super::path::parse_path;

pub fn parse_namespace(tokens: &mut Tokens, public: bool) -> NodeInfo {
    let root = match tokens.parse_identifier() {
        Some(s) => s,
        None => panic!(),
    };
    let path = parse_path(tokens, &root);
    return tokens.create_node(Node::NameSpace {
        public,
        static_path: path,
    });
}
