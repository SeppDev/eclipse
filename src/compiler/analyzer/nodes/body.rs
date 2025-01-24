use crate::compiler::{
    analyzer::types::ModuleTypes,
    errors::CompileCtx,
    nodes::{ast, hlir},
};

impl hlir::Function {
    pub fn handle_body(
        &mut self,
        ctx: &mut CompileCtx,
        types: &ModuleTypes,
        return_type: &Option<ast::Type>,
        mut body: Vec<ast::Node>,
    ) -> Vec<hlir::Node> {
        self.variables.push_scope();
        let mut nodes = Vec::new();
        loop {
            let node = match body.pop() {
                Some(n) => n,
                None => break,
            };

            let mut returned = false;
            let node: hlir::Node = match node.raw {
                ast::RawNode::Return(expression) => {
                    returned = true;
                    self.handle_return(ctx, types, expression, return_type)
                }
                ast::RawNode::DeclareVariable {
                    name,
                    mutable,
                    data_type,
                    expression,
                } => self.handle_decl(
                    ctx,
                    types,
                    node.position,
                    name,
                    mutable,
                    data_type,
                    expression,
                ),
                ast::RawNode::SetPath(path, expression) => {
                    self.handle_set(ctx, types, node.position, path, expression)
                }
                ast::RawNode::Loop { condition, body } => {
                    self.handle_loop(ctx, types, condition, body)
                }
                ast::RawNode::Scope(body) => {
                    hlir::Node::Scope(self.handle_body(ctx, types, return_type, body))
                }
                raw => {
                    ctx.error(node.position, format!("Not yet implemented: {raw:#?}"));
                    continue;
                }
            };

            nodes.push(node);
            if returned {
                break;
            }
        }

        self.variables.pop_scope();
        return nodes;
    }
}
