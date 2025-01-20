use crate::compiler::{
    analyzer::AnalyzedModule,
    errors::CompileCtx,
    nodes::{
        hlir,
        ir::{self, BinaryOperation, IRModule},
    },
};

use super::variables::VariablesMap;

mod body;
mod call;
mod expression;
mod result;
mod types;
mod variable;

impl ir::Function {
    // fn push(&mut self, instruction: ir::Instruction) {
    //     self.body.push_back(instruction)
    // }
    pub(super) fn label(&mut self, label: &String) {
        self.body.pushln(format!("{label}:"));
    }
    pub(super) fn store(&mut self, destination: &String, data_type: &ir::Type, value: &ir::Value) {
        self.body
            .tpushln(format!("store {data_type} {value}, ptr %{destination}"));
    }
    pub(super) fn store_pointer(
        &mut self,
        destination: &String,
        data_type: &ir::Type,
        pointer: &String,
    ) {
        self.body
            .tpushln(format!("store {data_type} %{pointer}, ptr %{destination}"));
    }
    pub(super) fn allocate(&mut self, destination: &String, data_type: &ir::Type) {
        self.body
            .tpushln(format!("%{destination} = alloca {data_type}"));
    }

    pub(super) fn load_pointer(
        &mut self,
        destination: &String,
        data_type: &ir::Type,
        pointer: &String,
    ) {
        self.body
            .tpushln(format!("%{destination} = load {data_type}, ptr %{pointer}"));
    }
    pub(super) fn load(&mut self, destination: &String, data_type: &ir::Type, value: &ir::Value) {
        self.body
            .tpushln(format!("%{destination} = load {data_type}, ptr {value}"));
    }
    pub(super) fn binary_operation(
        &mut self,
        destination: &String,
        data_type: &ir::Type,
        operation: &BinaryOperation,
        first: &ir::Value,
        second: &ir::Value,
    ) {
        self.body.tpushln(format!(
            "%{destination} = {operation} {data_type} {first}, {second}"
        ));
    }
    pub(super) fn r#return(&mut self, data_type: &ir::Type, value: Option<&ir::Value>) {
        match value {
            Some(val) => {
                self.body.tpushln(format!("ret {data_type} {val}"));
            }
            None => {
                self.body.tpushln(format!("ret {data_type}"));
            }
        }
    }
    pub(super) fn call(
        &mut self,
        destination: Option<&String>,
        data_type: &ir::Type,
        key: &String,
        arguments: Vec<(ir::Type, ir::Value)>,
    ) {
        let arguments = arguments
            .iter()
            .map(|(data_type, value)| format!("{data_type} {value}"))
            .collect::<Vec<String>>()
            .join(", ");

        match destination {
            Some(dest) => self
                .body
                .tpushln(format!("%{dest} = call {data_type} @{key}({arguments})")),
            None => self
                .body
                .tpushln(format!("call {data_type} @{key}({arguments})")),
        };
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
    function: hlir::Function,
) -> ir::Function {
    let key = function.key;

    let mut ir_function = ir::Function {
        key,
        return_type: ctx.target.convert(&function.return_type),
        parameters: Vec::new(),
        body: super::Source::new(),
        variables: VariablesMap::new(),
        old_variables: function.variables.map,
    };

    for parameter in function.parameters {
        let ir_type = ctx.target.convert(&parameter.data_type);

        if parameter.mutable {
            let param_name = ir_function.variables.increment();
            let loaded_key = ir_function.variables.generate();
            ir_function.allocate(&loaded_key, &ir_type);
            ir_function.store_pointer(&loaded_key, &ir_type, &param_name);

            ir_function.parameters.push((ir_type, param_name));

            ir_function
                .variables
                .insert(parameter.name, false)
                .to_string();
        } else {
            ir_function.variables.insert(parameter.name.clone(), true);
            ir_function.parameters.push((ir_type, parameter.name));
        }
    }

    ir_function.handle_body(ctx, function.body);

    return ir_function;
}
