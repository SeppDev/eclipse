use crate::compiler::{
    analyzer::types::ModuleTypes,
    errors::{CompileCtx, Location},
    nodes::{
        ast::{self, Identifier, Located, LocatedPath},
        hlir,
    },
};

impl hlir::Function {
    pub fn handle_decl(
        &mut self,
        ctx: &mut CompileCtx,
        types: &ModuleTypes,
        location: Location,
        name: Identifier,
        mutable: Option<Located<bool>>,
        data_type: Option<ast::Type>,
        expression: Option<ast::Expression>,
    ) -> hlir::Node {
        let expression = match expression {
            Some(expression) => expression,
            None => todo!(),
        };

        let data_type = self.infere_type(ctx, types, data_type, &expression);
        let expression = self.handle_expression(ctx, types, expression, data_type.clone());

        let key =
            match self
                .variables
                .insert(name.raw, mutable.is_some(), data_type.clone(), location)
            {
                Ok(k) => k,
                Err(old) => todo!(),
            };

        hlir::Node::DeclareVariable {
            name: key,
            data_type,
            expression,
        }
    }
    pub fn handle_set(
        &mut self,
        ctx: &mut CompileCtx,
        types: &ModuleTypes,
        location: Location,
        path: LocatedPath,
        expression: ast::Expression,
    ) -> hlir::Node {
        if path.raw.len() == 1 {
            if let Some(name) = path.raw.first() {
                match self.variables.read(name) {
                    Some(var) => {
                        var.modified = true;

                        if !var.mutable {
                            ctx.error(
                                location,
                                format!("Cannot modify immutable variable '{name}'"),
                            );
                        }

                        let data_type = var.data_type.clone();
                        let name = var.key.clone();

                        let expression =
                            self.handle_expression(ctx, types, expression, data_type.clone());

                        return hlir::Node::SetVariable { name, expression };
                    }
                    None => {}
                };
            }
        }

        ctx.error(location, format!("Cannot find value at path {}", path.raw));

        hlir::Node::default()
    }
}
