use crate::compiler::{
    analyzer::types::ModuleTypes,
    errors::CompileCtx,
    nodes::{ast, hlir},
};

impl hlir::Function {
    pub fn handle_loop(
        &mut self,
        ctx: &mut CompileCtx,
        types: &ModuleTypes,
        condition: Option<ast::Expression>,
        body: Vec<ast::Node>,
    ) -> hlir::Node {
        // return hlir::Node::Loop {
        //     condition: None,
        //     body: ,
        // };
        todo!()
    }
}
