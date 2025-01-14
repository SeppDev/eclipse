use crate::compiler::{
    errors::CompileCtx,
    nodes::{hlir, ir},
};

impl ir::Function {
    pub(super) fn handle_decl(
        &mut self,
        ctx: &mut CompileCtx,
        name: String,
        data_type: hlir::Type,
        expression: hlir::Expression,
    ) {
        let variable = self.old_variables.get(&name).unwrap();

        let destination = self.variables.generate();
        let mut is_register = false;

        let ir_type = ctx.target.convert(&data_type);

        match expression.raw {
            hlir::RawExpression::Call(..) => {
                if variable.mutable {
                    self.allocate(&destination, &ir_type);

                    let value = self.handle_expression(ctx, expression);
                    self.store(&destination, &ir_type, &value);
                } else {
                    is_register = true;

                    self.handle_store_expression(ctx, &destination, expression);
                }
            }
            _ => {
                self.allocate(&destination, &ir_type);

                let value = self.handle_expression(ctx, expression);
                self.store(&destination, &ir_type, &value);
            }
        }

        self.variables.insert(name, is_register);
    }
    pub(super) fn handle_set_variable(
        &mut self,
        ctx: &mut CompileCtx,
        name: String,
        expression: hlir::Expression,
    ) {
        let destination = self.variables.get(&name).key.clone();
        self.handle_store_expression(ctx, &destination, expression);
    }
}

// let value = match expression.raw {
//     hlir::RawExpression::GetVariable(name) => {
//         self.allocate(destination.clone(), ir_type.clone());

//         let variable = self.variables.get(&name);
//         if variable.is_register_value {
//             ir::Value::Register(variable.key.clone())
//         } else {
//             let key = ctx.counter.increment();
//             self.push(ir::Instruction::Define {
//                 destination: key.clone(),
//                 operation: ir::Operation::Load(
//                     ir_type.clone(),
//                     ir::Value::Reference(variable.key.clone()),
//                 ),
//             });

//             ir::Value::Reference(key)
//         }
//     }
//     hlir::RawExpression::Integer(value) => {
//         self.allocate(destination.clone(), ir_type.clone());
//         ir::Value::Integer(value)
//     }
//     hlir::RawExpression::Boolean(value) => {
//         self.allocate(destination.clone(), ir_type.clone());
//         ir::Value::Boolean(value)
//     }
//     hlir::RawExpression::Call(key, _) => {
//         self.variables.insert(name, true);

//         self.push(ir::Instruction::Define {
//             destination,
//             operation: ir::Operation::Call(ir_type, key, Vec::new()),
//         });
//         return;
//     }
//     _ => todo!(),
// };
