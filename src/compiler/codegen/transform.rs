use crate::compiler::{
    analyzer::AnalyzedModule,
    errors::CompileCtx,
    nodes::{
        hlir,
        ir::{self, IRModule},
    },
};

mod types;

pub fn transform(ctx: &CompileCtx, mut module: AnalyzedModule) -> IRModule {
    let mut ir_module = IRModule::default();

    loop {
        let function = match module.functions.pop() {
            Some(f) => handle_function(ctx, &mut ir_module, f),
            None => break,
        };
        ir_module.functions.push(function)
    }

    return ir_module;
}

fn handle_function(
    ctx: &CompileCtx,
    module: &mut IRModule,
    mut function: hlir::Function,
) -> ir::Function {
    let key = function.key.drain(..).collect::<String>();

    let mut ir_function = ir::Function {
        key,
        return_type: function.return_type.convert(),
        parameters: Vec::new(),
        body: Vec::new(),
    };

    loop {
        let node = match function.body.pop() {
            Some(n) => n,
            None => break,
        };

        match node {
            hlir::Node::Return(data_type, expression) => {
                handle_return(ctx, &mut ir_function, data_type, expression)
            }
            _ => todo!(),
        }
    }

    return ir_function;
}

fn handle_return(
    ctx: &CompileCtx,
    ir_function: &mut ir::Function,
    data_type: hlir::Type,
    expression: Option<hlir::Expression>,
) {
    let expression = match expression {
        Some(expr) => expr,
        None => {
            return ir_function
                .body
                .push(ir::Instruction::Return(data_type.convert(), None))
        }
    };

    ir_function
        .body
        .push(ir::Instruction::Return(data_type.convert(), None))
}
