use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult, Location},
    parser::{Node, NodeInfo, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::{ReferenceManager, Type},
};

use super::{
    variables::VariablesMap, FileTypes, IRFunction, IRProgram, IRType, IRValue, Operation,
};

pub struct ProgramCtx<'a> {
    pub debug: &'a mut CompileCtx,
    pub functions: &'a mut Vec<IRFunction>,
    pub types: &'a FileTypes,
    count: &'a mut NameCounter,
    static_strings: &'a mut Vec<(String, String)>,
}
impl<'a> ProgramCtx<'a> {
    pub fn push_string(&mut self, string: String) -> String {
        let key = self.count.increment();
        self.static_strings.push((key.clone(), string));
        return key;
    }
}

pub struct FunctionCtx<'a> {
    pub variables: VariablesMap,
    pub return_type: &'a Option<Type>,
    pub operations: Vec<Operation>,
    pub relative_path: &'a Path,
}

pub fn analyze(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    types: FileTypes,
    mut program: ParsedProgram,
) -> CompileResult<IRProgram> {
    let mut functions = Vec::new();
    // let std_path = Path::from("std");
    // analyze_file(parsed, &mut functions, errors, &parsed.standard, &std_path);

    let mut static_strings = Vec::new();

    let mut ctx = ProgramCtx {
        debug,
        count,
        functions: &mut functions,
        types: &types,
        static_strings: &mut static_strings,
    };

    handle_file(&mut ctx, &mut program.main);

    return Ok(IRProgram {
        functions,
        static_strings,
    });
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
    parameters: Vec<(String, Type)>,
    return_type: Type,
    body: Vec<NodeInfo>,
) {
    let mut variables = VariablesMap::new();
    variables.push_scope();

    let mut new_params = Vec::new();
    for (name, data_type) in parameters {
        let param_key = variables.increment();

        new_params.push((param_key.clone(), data_type.convert()));
        variables.insert(true, &name, false, data_type, location.clone());

        variables.set_key(&name, param_key);
    }

    // let missing_return = match body.last() {
    //     Some(info) => match info.node {
    //         Node::Return(_) => false,
    //         _ => !return_type.is_void(),
    //     },
    //     None => !return_type.is_void(),
    // };

    // if missing_return {
    //     program.debug.error(location, "Missing return");
    // }

    let ir_type = return_type.convert();
    let mut function = FunctionCtx {
        variables,
        return_type: &Some(return_type),
        relative_path: &file.relative_path,
        operations: Vec::new(),
    };

    handle_body(program, &mut function, body);

    function.variables.pop_scope();

    if !matches!(
        function.operations.last().unwrap_or(&Operation::Unkown),
        Operation::Return {
            data_type: _,
            value: _
        }
    ) {
        function.operations.push(Operation::Return {
            data_type: IRType::Void,
            value: IRValue::Null,
        })
    }

    program.functions.push(IRFunction {
        name: key,
        operations: function.operations,
        parameters: new_params,
        return_type: ir_type,
    });
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
            Node::Call(path, arguments) => {
                handle_call(program, function, info.location, path, arguments)
            }
            Node::Return(expression) => handle_return(program, function, info.location, expression),
            _ => program
                .debug
                .result_print(format!("Todo: {:#?}", info.node)),
        }
    }

    function.variables.pop_scope();
}
