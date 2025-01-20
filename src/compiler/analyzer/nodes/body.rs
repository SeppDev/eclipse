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
        body: Vec<ast::Node>,
    ) -> bool {
        loop {
            let node = match body.pop() {
                Some(n) => n,
                None => break false,
            };

            let mut returned = false;
            let node: hlir::Node = match node.raw {
                ast::RawNode::Return(expression) => {
                    returned = true;
                    self.handle_return(ctx, types, expression, Some(self.raw.return_type.clone()))
                }
                ast::RawNode::DeclareVariable {
                    name,
                    mutable,
                    data_type,
                    expression,
                } => self.handle_decl(
                    ctx,
                    types,
                    node.location,
                    name,
                    mutable,
                    data_type,
                    expression,
                ),
                ast::RawNode::SetPath(path, expression) => {
                    self.handle_set(ctx, types, node.location, path, expression)
                }
                ast::RawNode::Loop { condition, body } => {
                    self.handle_loop(ctx, types, condition, body)
                }
                raw => {
                    ctx.error(node.location, format!("Not yet implemented: {raw:#?}"));
                    continue;
                }
            };

            self.body.push(node);
            if returned {
                break returned;
            }
        }
    }
}
