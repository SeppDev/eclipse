use context::CompilerCtx;
use syntax::{hlir, mir};

pub fn lower_to_mir(compiler: &CompilerCtx, collection: hlir::ModuleCollection) -> mir::Module {
    let mut nodes = Vec::new();

    for module in collection.modules {
        let body = module.nodes.into_iter().map(|n| lower_node(n));
        nodes.extend(body);
    }

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
            node,
        } => mir::Node::Function {
            name,
            parameters: Vec::new(),
            return_type: mir::Type::Void,
            body: node_to_body(*node),
        },
        r => todo!("{r:?}"),
    }
}

fn lower_expression(node: hlir::Node) -> mir::Expression {
    use hlir::Node;
    use mir::Expression;

    match node {
        Node::Integer(n) => Expression::Integer(n),
        r => todo!("{r:?}"),
    }
}

fn node_to_body(node: hlir::Node) -> Vec<mir::Node> {
    use hlir::Node;

    match node {
        Node::Block(body) => body.into_iter().map(|n| lower_node(n)).collect(),
        _ => vec![lower_node(node)],
    }
}
