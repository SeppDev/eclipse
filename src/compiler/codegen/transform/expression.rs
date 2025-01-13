use crate::compiler::{
    errors::CompileCtx,
    nodes::{hlir, ir},
};

impl ir::Function {
    pub(super) fn handle_expression(
        &mut self,
        ctx: &mut CompileCtx,
        destination: String, 
        data_type: hlir::Type,
        expression: hlir::Expression,
    ){
        match expression.raw {
            _ => todo!(),
        }
    }
}