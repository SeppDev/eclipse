use crate::compiler::{errors::{CompileCtx, Location}, nodes::{ast::{self, Identifier, Located}, hlir}};


impl hlir::Function {
    pub fn handle_decl(
        &mut self,
        ctx: &mut CompileCtx,
        location: Location,
        name: Identifier,
        mutable: Option<Located<bool>>,
        data_type: Option<ast::Type>,
        expression: Option<ast::Expression>,
    ) -> hlir::Node {
        let data_type = self.infere_type(ctx, &data_type, &expression);

        let expression = match expression {
            Some(expression) => expression,
            None => todo!(),
        };
        
        let expression = self.handle_expression(ctx, expression, data_type.clone());
        let key = ctx.counter.increment();

        self.variables.insert(name.raw.clone(), key.clone(), mutable.is_some(), data_type.clone(), location);
        
        hlir::Node::DeclareVariable {
            name: key,
            mutable: mutable.is_some(),
            data_type,
            expression,
        }
    }
}
