use crate::compiler::{
    analyzer::AnalyzedModule,
    errors::CompileCtx,
    nodes::{
        ir::{self, IRModule},
        mir,
    },
};

pub fn transform(ctx: &CompileCtx, mut module: AnalyzedModule) -> IRModule {
    let mut ir_module = IRModule::default();

    loop {
        let function = match module.functions.pop() {
            Some(f) => handle_function(ctx, f),
            None => break,
        };
        ir_module.functions.push(function)
    }

    return ir_module;
}

fn handle_function(ctx: &CompileCtx, function: mir::Function) -> ir::Function {
    let mut ir_function = ir::Function {
        key: function.key,
        return_type: ir::Type::Void,
        parameters: Vec::new(),
        body: Vec::new(),
    };

    for node in function.body {
        match node {
            mir::Node::Return(data_type, expression) => ir_function
                .body
                .push(ir::Instruction::Return(ir::Type::Void, None)),
            _ => todo!(),
        }
    }

    return ir_function;
}
