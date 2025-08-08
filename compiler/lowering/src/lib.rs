use context::CompilerCtx;
use syntax::{hlir, mir};

mod function;

struct Lowering {
    functions: Vec<mir::Function>,
}

pub fn lower_to_mir(compiler: &CompilerCtx, collection: hlir::ModuleCollection) -> mir::Module {
    todo!()
}

impl Lowering {
    fn lower_node(&mut self, node: hlir::Node) {
        use hlir::Node;

        match node {
            r => todo!("{r:?}"),
        }
    }

    fn lower_expression(&self, node: hlir::Node) -> mir::Expression {
        use hlir::Node;
        use mir::Expression;

        match node {
            Node::Integer(n) => Expression::Integer(mir::Type::Int(32), n),
            r => todo!("{r:?}"),
        }
    }

    fn node_to_body(&self, node: hlir::Node) {
        use hlir::Node;

        match node {
            _ => todo!("{node:#?}"),
        }
    }
}
