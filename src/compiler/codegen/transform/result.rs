use crate::compiler::{
    errors::CompileCtx,
    nodes::{hlir, ir},
};

impl ir::Function {
    pub(super) fn handle_return(
        &mut self,
        ctx: &mut CompileCtx,
        data_type: hlir::Type,
        expression: Option<hlir::Expression>,
    ) {
        let expression = match expression {
            Some(expr) => expr,
            None => {
                return self.push(ir::Instruction::Return(
                    ctx.target.convert(&data_type),
                    None,
                ));
            }
        };

        let ir_type = ctx.target.convert(&data_type);

        let value: ir::Value = match expression.raw {
            hlir::RawExpression::GetVariable(name) => {
                let variable = self.variables.get(&name);
                if variable.is_register_value {
                    ir::Value::Reference(variable.key.clone())
                } else {
                    let key = ctx.counter.increment();
                    self.push(ir::Instruction::Define {
                        destination: key.clone(),
                        operation: ir::Operation::Load(ir_type.clone(), ir::Value::Reference(variable.key.clone())),
                    });

                    ir::Value::Reference(key)
                }
            }
            hlir::RawExpression::Integer(value) => ir::Value::Integer(value),
            _ => todo!(),
        };

        self.push(ir::Instruction::Return(ir_type, Some(value)));
    }
}
