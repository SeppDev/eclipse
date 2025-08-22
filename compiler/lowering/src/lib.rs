use context::CompilerCtx;
use syntax::{hir, mir};

mod function;

struct Lowering {
    functions: Vec<mir::Function>,
}

pub fn lower_to_mir(compiler: &CompilerCtx, collection: hir::ModuleCollection) -> mir::Module {
    todo!()
}

impl Lowering {
    fn lower_node(&mut self, node: hir::Node) {
        use hir::Node;

        match node {
            r => todo!("{r:?}"),
        }
    }

    fn lower_expression(&self, node: hir::Node) -> mir::Expression {
        use hir::Node;
        use mir::Expression;

        match node {
            Node::Integer(n) => Expression::Integer(mir::Type::Int(32), n),
            r => todo!("{r:?}"),
        }
    }

    fn node_to_body(&self, node: hir::Node) {
        use hir::Node;

        match node {
            _ => todo!("{node:#?}"),
        }
    }
}
