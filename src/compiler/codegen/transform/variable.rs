use crate::compiler::{
    errors::CompileCtx,
    nodes::{hlir, ir},
};

impl ir::Function {
    pub(super) fn handle_decl(
        &mut self,
        ctx: &mut CompileCtx,
        name: String,
        mutable: bool,
        data_type: hlir::Type,
        expression: hlir::Expression,
    ) {
        let ir_type = ctx.target.convert(&data_type);
        let destination = self.variables.generate();

        let value = match expression.raw {
            hlir::RawExpression::GetVariable(name) => {
                self.allocate(destination.clone(), ir_type.clone());

                let variable = self.variables.get(&name);
                if variable.is_register_value {
                    ir::Value::Register(variable.key.clone())
                } else {
                    let key = ctx.counter.increment();
                    self.push(ir::Instruction::Define {
                        destination: key.clone(),
                        operation: ir::Operation::Load(
                            ir_type.clone(),
                            ir::Value::Reference(variable.key.clone()),
                        ),
                    });

                    ir::Value::Reference(key)
                }
            }
            hlir::RawExpression::Integer(value) => {
                self.allocate(destination.clone(), ir_type.clone());
                ir::Value::Integer(value)
            }
            hlir::RawExpression::Boolean(value) => {
                self.allocate(destination.clone(), ir_type.clone());
                ir::Value::Boolean(value)
            }
            hlir::RawExpression::Call(key, _) => {
                self.variables.insert(name, true);

                self.push(ir::Instruction::Define {
                    destination,
                    operation: ir::Operation::Call(ir_type, key, Vec::new()),
                });
                return;
            }
            _ => todo!(),
        };

        self.variables.insert(name, false);
 
        self.push(ir::Instruction::Store {
            data_type: ir_type,
            value,
            pointer: ir::Value::Reference(destination.clone()),
        });
    }
}
