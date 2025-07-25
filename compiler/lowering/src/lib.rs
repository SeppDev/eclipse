use syntax::{hlir, mir};

pub fn lower(module: &hlir::ModuleCollection) -> mir::Module {
    use hlir::Node;

    let nodes = {};

    mir::Module { nodes }
}

fn lower_node(node: hlir::Node) -> mir::Node {
    use hlir::Node;

    match node {
        Node::Return(_) => mir::Node::Return(None),
        Node::Block(body) => mir::Node::Block(body.into_iter().map(|n| lower_node(n)).collect()),
        Node::Function {
            name,
            parameters,
            return_type,
            body,
        } => mir::Node::Function {
            name,
            parameters: Vec::new(),
            return_type: mir::Type::Void,
            body: Box::new(lower_node(*body)),
        },
        r => todo!("{r:?}"),
    }
}
 