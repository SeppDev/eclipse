use crate::{
    parser::{ASTNode, Node, Program},
    CompileError,
};


pub fn analyze(program: Program) -> Result<(), CompileError> {
    // println!("{:#?}", program);

    for (path, mut module) in program.modules {
        match parse_root(&mut module.body) {
            Ok(_) => {},
            Err(error) => return Err(error) 
        };
    }

    todo!()
}

pub fn parse_root(nodes: &mut Vec<ASTNode>) -> Result<(), CompileError> {
    for ast_node in nodes {
        let node = &ast_node.node;
        match node {
            Node::Function {
                export,
                is_unsafe,
                name,
                parameters,
                return_type,
                body,
            } => {
                
            }
            Node::Import(_, _) => continue,
            _ => panic!("Expected function"), // _ => Err(CompileError::BuildProblem(BuildProblem::new(BuildError::, relative_path, line)))
        }
    }

    return Ok(());
}
