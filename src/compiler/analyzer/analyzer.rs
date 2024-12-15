use crate::compiler::{
    codegen::{CodeGen, FunctionOperations},
    errors::{CompileCtx, CompileResult, Location},
    parser::{Node, NodeInfo, Parameter, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
    FILE_EXTENSION,
};

use super::{variables::VariablesMap, IRType, IRValue, ProgramTypes};

#[derive(Debug)]
pub struct ProgramCtx<'a> {
    pub debug: &'a mut CompileCtx,
    pub codegen: CodeGen,
    pub types: &'a ProgramTypes,
    pub namespaces: &'a mut Vec<Path>, // pub count: &'a mut NameCounter,
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
    pub variables: &'a mut VariablesMap,
    pub return_type: &'a Option<Type>,
    pub operations: &'a mut FunctionOperations,
    pub relative_path: &'a Path,
    pub loop_info: Vec<LoopInfo>,
}

pub fn analyze(program: &mut ProgramCtx, mut parsed: ParsedProgram) -> CompileResult<()> {
    handle_file(program, &mut parsed.main);
    handle_file(program, &mut parsed.standard);

    // program.debug.result_print(format!("{program:#?}"));

    return Ok(());
}

fn handle_file(program: &mut ProgramCtx, file: &mut ParsedFile) {
    program.debug.set_status(format!(
        "Analyzing: {}.{FILE_EXTENSION}",
        file.relative_file_path.convert().to_string_lossy()
    ));

    program.namespaces.clear();
    program
        .namespaces
        .push(Path::from("std").join("io").join("print"));

    program.debug.set_path(&file.relative_file_path);

    for info in &file.body {
        match &info.node {
            Node::NameSpace {
                public: _,
                static_path,
            } => program.namespaces.push(static_path.clone()),
            _ => continue,
        }
    }

    loop {
        let info = match file.body.pop() {
            Some(f) => f,
            None => break,
        };

        if let Node::Function {
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
            )
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
        let is_basic = parameter.data_type.base.is_basic();
        let ir_type = if is_basic {
            parameter.data_type.convert()
        } else {
            IRType::Pointer
        };

        if parameter.mutable && is_basic {
            if parameter.data_type.is_pointing() {
                program.debug.error(
                    parameter.location.clone(),
                    format!("A parameter cannot be mutable if it is a pointer or reference."),
                );
            }

            let key = variables.increment();
            new_params.push((key.clone(), ir_type));
            mutables.push((parameter.name, key, parameter.data_type));
            continue;
        }
        let variable = variables.insert(
            false,
            parameter.name,
            false,
            parameter.data_type,
            parameter.location,
        );
        new_params.push((variable.key.clone(), ir_type));
    }

    let mut operations = FunctionOperations::new(&key, &return_type, &new_params);

    for (name, key, data_type) in mutables {
        new_params.push((key.clone(), data_type.convert()));

        let param_variable =
            variables.insert(true, name, true, data_type.clone(), location.clone());

        operations.allocate(&param_variable.key, &data_type.convert());
        operations.store(
            &data_type.convert(),
            &IRValue::Variable(key),
            &param_variable.key,
        );
    }

    let returns_void = return_type.base.is_void();
    let return_expected = format!("Return expected with type {return_type}");

    let mut function = FunctionCtx {
        variables: &mut variables,
        return_type: &Some(return_type),
        relative_path: &file.relative_path,
        operations: &mut operations,
        loop_info: Vec::new(),
    };

    let returned = handle_body(program, &mut function, body);

    function.variables.pop_scope();

    if returns_void {
        if !returned {
            operations.void_return();
        }
    } else {
        if !returned {
            program.debug.error(location, return_expected);
        }
    }

    program.codegen.insert(operations);
}

mod nodes;
use nodes::*;
mod types;
use types::*;

fn handle_body(program: &mut ProgramCtx, function: &mut FunctionCtx, nodes: Vec<NodeInfo>) -> bool {
    function.variables.push_scope();

    let mut namespaces = 0;
    for info in &nodes {
        match &info.node {
            Node::NameSpace {
                public: _,
                static_path,
            } => program.namespaces.push(static_path.clone()),
            _ => continue,
        }
        namespaces += 1;
    }

    let mut returned = false;

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
                body,
                elseif: _,
                else_body,
            } => handle_ifstatement(
                program,
                function,
                info.location,
                expression,
                body,
                else_body,
            ),
            Node::SetVariable { name, expression } => {
                handle_set_variable(program, function, info.location, name, expression)
            }

            Node::Call(path, arguments) => {
                handle_call(program, function, None, &info.location, path, arguments)
            }
            Node::Break => handle_break(program, function, info.location),
            Node::Continue => handle_continue(program, function, info.location),
            Node::Scope(body) => returned = handle_body(program, function, body),

            Node::Return(expression) => {
                handle_return(
                    program,
                    function,
                    info.location,
                    function.return_type,
                    expression,
                );
                returned = true;
                break;
            }
            Node::NameSpace {
                public: _,
                static_path: _,
            } => continue,
            _ => program
                .debug
                .result_print(format!("Todo: {:#?}", info.node)),
        }
    }

    function.variables.pop_scope();

    for _ in 0..namespaces {
        program.namespaces.pop();
    }

    return returned;
}
