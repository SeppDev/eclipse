use crate::compiler::{
    errors::CompileCtx,
    nodes::{hlir, ir},
};

impl ir::Function {
    pub(super) fn handle_body(&mut self, ctx: &mut CompileCtx, body: Vec<hlir::Node>) {
        let mut nodes = body.into_iter();
        loop {
            let node = match nodes.next() {
                Some(n) => n,
                None => break,
            };
            match node {
                hlir::Node::DeclareVariable {
                    name,
                    data_type,
                    expression,
                } => self.handle_decl(ctx, name, data_type, expression),
                hlir::Node::Return(data_type, expression) => {
                    self.handle_return(ctx, data_type, expression)
                }
                hlir::Node::SetVariable { name, expression } => {
                    self.handle_set_variable(ctx, name, expression)
                }
                hlir::Node::Scope(body) => self.handle_body(ctx, body),
                _ => todo!(),
            }
        }
    }
}
