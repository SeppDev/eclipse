use crate::compiler::{
    codegen::{CodeGen, FunctionOperations},
    errors::{CompileCtx, CompileResult, Location},
    parser::{Node, NodeInfo, Parameter, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
};

use super::{variables::VariablesMap, FileTypes, IRValue};

pub struct ProgramCtx<'a> {
    pub debug: &'a mut CompileCtx,
    pub codegen: CodeGen,
    pub types: &'a FileTypes,
    // pub count: &'a mut NameCounter,
    // pub static_strings: &'a mut Vec<(String, String)>,
}
// impl<'a> ProgramCtx<'a> {
//     pub fn push_string(&mut self, string: String) -> String {
//         let key = self.count.increment();
//         self.static_strings.push((key.clone(), string));
//         return key;
//     }
// }

pub struct LoopInfo {
    pub begin: String,
    pub end: String,
}
impl LoopInfo {
    pub fn new<T: ToString, E: ToString>(begin: T, end: E) -> Self {
        Self {
            begin: begin.to_string(),
            end: end.to_string(),
        }
    }
}

pub struct FunctionCtx<'a> {
    pub variables: VariablesMap,
    pub return_type: &'a Option<Type>,
    pub operations: &'a mut FunctionOperations,
    pub relative_path: &'a Path,
    pub loop_info: Vec<LoopInfo>,
}

pub fn analyze(program: &mut ProgramCtx, mut parsed: ParsedProgram) -> CompileResult<()> {
    // let std_path = Path::from("std");
    // analyze_file(parsed, &mut functions, errors, &parsed.standard, &std_path);

    handle_file(program, &mut parsed.main);

    return Ok(());
}

fn handle_file(program: &mut ProgramCtx, file: &mut ParsedFile) {
    program
        .debug
        .set_status(format!("Analyzing: {}", file.relative_file_path));

    program.debug.set_path(&file.relative_file_path);

    loop {
        let info = match file.body.pop() {
            Some(f) => f,
            None => break,
        };

        if let Node::Function {
            export: _,
            name: _,
            key,
            parameters,
            return_type,
            body,
        } = info.node
        {
            handle_function(
                program,
                &file,
                info.location,
                key,
                parameters,
                return_type,
                body,
            );
            continue;
        }
    }

    loop {
        let (_, mut import) = match file.imports.pop_first() {
            Some(f) => f,
            None => break,
        };
        handle_file(program, &mut import);
    }
}

fn handle_function(
    program: &mut ProgramCtx,
    file: &ParsedFile,
    location: Location,

    key: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Vec<NodeInfo>,
) {
    let mut variables = VariablesMap::new();
    variables.push_scope();

    let mut mutables = Vec::new();
    let mut new_params = Vec::new();
    for parameter in parameters {
        if parameter.mutable {
            if parameter.data_type.is_pointing() {
                program.debug.error(
                    location.clone(),
                    format!(
                        "A parameter cannot be mutable if it is a pointer or reference."
                    ),
                );
            }

            let key = variables.increment();
            new_params.push((key.clone(), parameter.data_type.convert()));
            mutables.push((parameter.name, key, parameter.data_type));
            continue;
        }

        let ir = parameter.data_type.convert();
        let variable = variables.insert(true, &parameter.name, false, parameter.data_type, location.clone());
        new_params.push((variable.key.clone(), ir));
    }

    let mut operations = FunctionOperations::new(&key, &return_type, &new_params);

    for (name, key, data_type) in mutables {
        new_params.push((key.clone(), data_type.convert()));

        let param_variable =
            variables.insert(false, &name, true, data_type.clone(), location.clone());

        operations.allocate(&param_variable.key, &data_type.convert());
        operations.store(
            &data_type.convert(),
            &IRValue::Variable(key),
            &param_variable.key,
        );
    }

    let mut function = FunctionCtx {
        variables,
        return_type: &Some(return_type),
        relative_path: &file.relative_path,
        operations: &mut operations,
        loop_info: Vec::new(),
    };

    handle_body(program, &mut function, body);

    function.variables.pop_scope();

    program.codegen.insert(operations);

    // if !matches!(
    //     function.operations.last().unwrap_or(&Operation::Unkown),
    //     Operation::Return {
    //         data_type: _,
    //         value: _
    //     }
    // ) {
    //     function.operations.push(Operation::Return {
    //         data_type: IRType::Void,
    //         value: IRValue::Null,
    //     })
    // }

    // program.functions.push(IRFunction {
    //     name: key,
    //     operations: function.operations,
    //     parameters: new_params,
    //     return_type: ir_type,
    // });
}

mod nodes;
use nodes::*;
mod expressions;
use expressions::*;

fn handle_body(program: &mut ProgramCtx, function: &mut FunctionCtx, nodes: Vec<NodeInfo>) {
    function.variables.push_scope();

    for info in nodes {
        match info.node {
            Node::DeclareVariable {
                name,
                mutable,
                data_type,
                expression,
            } => handle_variable_declaration(
                program,
                function,
                info.location,
                name,
                mutable,
                data_type,
                expression,
            ),
            Node::Loop { condition, body } => {
                handle_loop(program, function, info.location, condition, body)
            }
            Node::IfStatement {
                expression,
                elseif: _,
                else_body,
            } => handle_ifstatement(
                program,
                function,
                info.location,
                expression.0,
                expression.1,
                else_body,
            ),
            Node::SetVariable { name, expression } => {
                handle_set_variable(program, function, info.location, name, expression)
            }
            Node::Call(path, arguments) => {
                handle_call(program, function, info.location, path, arguments)
            }
            Node::Break => handle_break(program, function, info.location),
            Node::Continue => handle_continue(program, function, info.location),
            Node::Scope(body) => handle_body(program, function, body),
            Node::Return(expression) => {
                handle_return(
                    program,
                    function,
                    info.location,
                    function.return_type,
                    expression,
                );
                break;
            }
            _ => program
                .debug
                .result_print(format!("Todo: {:#?}", info.node)),
        }
    }

    function.variables.pop_scope();
}
