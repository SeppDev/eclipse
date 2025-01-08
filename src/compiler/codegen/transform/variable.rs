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
        let destination = name;

        let value: ir::Value = match expression.raw {
            hlir::RawExpression::Integer(value) => ir::Value::Integer(value),
            _ => todo!(),
        };
        
        self.allocate(destination.clone(), ir_type.clone());
        
        self.push(ir::Instruction::Store {
            data_type: ir_type,
            value,
            pointer: ir::Value::Reference(destination.clone()),
        });
    
        
    }
}

