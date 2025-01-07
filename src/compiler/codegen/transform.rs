use std::collections::VecDeque;

use crate::compiler::{
    analyzer::AnalyzedModule,
    errors::CompileCtx,
    nodes::{
        hlir,
        ir::{self, IRModule},
    },
};

mod result;
mod types;
mod variable;

impl ir::Function {
    fn push(&mut self, instruction: ir::Instruction) {
        self.body.push_back(instruction)
    }
    fn allocate(&mut self, destination: String, data_type: ir::Type) {
        self.body.push_front(ir::Instruction::Define {
            destination,
            operation: ir::Operation::Allocate(data_type),
        })
    }
}

pub fn transform(ctx: &mut CompileCtx, mut module: AnalyzedModule) -> IRModule {
    let mut ir_module: IRModule = IRModule::default();

    loop {
        let function: ir::Function = match module.functions.pop() {
            Some(function) => handle_function(ctx, &mut ir_module, function),
            None => break,
        };
        ir_module.functions.push(function);
    }

    return ir_module;
}

fn handle_function(
    ctx: &mut CompileCtx,
    module: &mut IRModule,
    mut function: hlir::Function,
) -> ir::Function {
    let key = function.key.drain(..).collect::<String>();

    let mut ir_function = ir::Function {
        key,
        return_type: ctx.target.convert(&function.return_type),
        parameters: Vec::new(),
        body: VecDeque::new(),
    };
    
    let mut nodes = function.body.into_iter();
    
    loop {
        let node = match nodes.next() {
            Some(n) => n,
            None => break,
        };
        
        match node {
            hlir::Node::DeclareVariable {
                key,
                mutable,
                data_type,
                expression,
            } => ir_function.handle_decl(ctx, key, mutable, data_type, expression),
            hlir::Node::Return(data_type, expression) => {
                ir_function.handle_return(ctx, data_type, expression)
            }

            _ => todo!(),
        }
    }

    return ir_function;
}
