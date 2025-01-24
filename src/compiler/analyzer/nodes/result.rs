use crate::compiler::{
    analyzer::types::ModuleTypes,
    errors::CompileCtx,
    nodes::{ast, hlir},
};

impl hlir::Function {
    pub fn handle_return(
        &mut self,
        ctx: &mut CompileCtx,
        types: &ModuleTypes,
        expression: Option<ast::Expression>,
        return_type: &Option<ast::Type>,
    ) -> hlir::Node {
        match expression {
            Some(expression) => {
                let data_type = self.infere_type(ctx, types, return_type, &expression);

                let expression = self.handle_expression(ctx, types, expression, data_type.clone());
                hlir::Node::Return(data_type, Some(expression))
            }
            None => {
                let data_type = match return_type {
                    Some(return_type) => {
                        let converted = return_type.raw.convert();
                        if converted != hlir::Type::Void {
                            ctx.error(return_type.location.clone(), "Missing expression");
                        }
                        converted
                    }
                    None => hlir::Type::Void,
                };

                hlir::Node::Return(data_type, None)
            }
        }
    }
}
