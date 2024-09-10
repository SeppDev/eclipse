use crate::parser::ASTNode;


#[derive(Debug)]
pub struct Module {
    // name: String,
    pub body: Vec<ASTNode>,
}

