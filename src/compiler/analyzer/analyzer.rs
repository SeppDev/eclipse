use crate::compiler::{
    errors::{CompileResult, Location},
    parser::{Node, NodeInfo, Parameter, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::Type,
    FILE_EXTENSION,
};

mod program;
pub use program::ProgramCtx;

mod function;
pub use function::{FunctionCtx, LoopInfo};

use super::{IRType, IRValue};

pub fn analyze(program: &mut ProgramCtx, mut parsed: ParsedProgram) -> CompileResult<()> {
    handle_file(program, &mut parsed.main);
    handle_file(program, &mut parsed.standard);

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
    let mut new_params = Vec::new();
    let mut mutables = Vec::new();
    let mut operations = program
        .codegen
        .new_function(&key, &return_type, &new_params);

    let mut function = FunctionCtx::new(
        Some(return_type.clone()),
        &mut operations,
        &file.relative_path,
    );
    function.push_vars_scope();

    for parameter in parameters {
        let is_basic = parameter.data_type.base.is_basic();
        let ir_type = if is_basic {
            parameter.data_type.convert()
        } else {
            IRType::Pointer
        };

        if parameter.mutable && is_basic {
            if parameter.data_type.pointers() > 0 {
                program.debug.error(
                    parameter.location.clone(),
                    format!("A parameter cannot be mutable if it is a pointer or reference."),
                );
            }

            let key = function.increment_key();
            new_params.push((key.clone(), ir_type));
            mutables.push((parameter.name, key, parameter.data_type));
            continue;
        }

        let key = function.increment_key();
        function.insert_variable(
            parameter.name,
            Some(key.clone()),
            false,
            parameter.data_type,
            parameter.location,
        );
        new_params.push((key, ir_type));
    }

    for (name, key, data_type) in mutables {
        new_params.push((key.clone(), data_type.convert()));

        let param_key = function.increment_key();
        function.insert_variable(
            name,
            Some(param_key.clone()),
            true,
            data_type.clone(),
            location.clone(),
        );

        function
            .operations
            .store(&data_type.convert(), &IRValue::Variable(key), &param_key);
    }

    let returns_void = return_type.base.is_void();
    let return_expected = format!("Return expected with type {return_type}");

    let returned = handle_body(program, &mut function, &Some(return_type), body);

    function.pop_vars_scope();

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

mod expression;
pub use expression::*;

mod types;
use types::*;

fn handle_body(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    return_type: &Option<Type>,
    nodes: Vec<NodeInfo>,
) -> bool {
    function.push_vars_scope();

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
        function.operations.new_line();
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
            Node::Loop { condition, body } => handle_loop(
                program,
                function,
                return_type,
                info.location,
                condition,
                body,
            ),
            Node::IfStatement {
                expression,
                body,
                elseif: _,
                else_body,
            } => handle_ifstatement(
                program,
                function,
                return_type,
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
            Node::Scope(body) => returned = handle_body(program, function, return_type, body),
            Node::Return(expression) => {
                handle_return(program, function, info.location, &function.return_type.clone(), expression);
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

    function.pop_vars_scope();

    for _ in 0..namespaces {
        program.namespaces.pop();
    }

    return returned;
}
