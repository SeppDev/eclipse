use crate::compiler::{
    analyzer::{analyzer::what_type, handle_read, FunctionCtx, IRType, IRValue, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
    path::Path,
    types::Type,
};

pub fn handle_call(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    destination: Option<(&String, &Type)>,
    location: &Location,
    path: Path,
    mut arguments: Vec<ExpressionInfo>,
) {
    let found = match program
        .types
        .get_function(function.relative_path, &path, program.namespaces)
    {
        Some(f) => f,
        None => {
            program.debug.error(
                location.clone(),
                format!("Could not find function: '{path}'"),
            );
            return;
        }
    };

    if arguments.len() != found.parameters.len() {
        program.debug.error(
            location.clone(),
            format!(
                "Expected {} arguments, but got {}",
                found.parameters.len(),
                arguments.len()
            ),
        );
        return;
    }

    arguments.reverse();

    let is_basic = found.return_type.base.is_basic();
    let return_type = if is_basic {
        &found.return_type
    } else {
        &Type::void()
    };

    let mut ir_arguments = Vec::new();
    let mut return_pointers = Vec::new();

    for param_type in &found.parameters {
        let info = arguments.pop().unwrap();

        let data_type = what_type(program, function, &info.location, Some(param_type), &info);
        let value = handle_read(program, function, &location, &data_type, info);

        let ir_type = if param_type.base.is_basic() {
            param_type.convert()
        } else {
            IRType::Pointer
        };

        ir_arguments.push((ir_type, value));
    }

    if let Some((destination, data_type)) = destination {
        if data_type != &found.return_type {
            program.debug.error(
                location.clone(),
                format!("Expected {data_type} but found {}", found.return_type),
            );
            return;
        }

        if is_basic {
            function.operations.store_call(
                &destination,
                &found.key,
                &return_type.convert(),
                IRValue::Arguments {
                    return_pointers,
                    arguments: ir_arguments,
                },
            );
            
            return;
        }
        
        function.operations.allocate(&destination, &data_type.convert());
        return_pointers.push((data_type.convert(), destination.clone()))
    }

    function.operations.call(
        &found.key,
        &return_type.convert(),
        IRValue::Arguments {
            return_pointers,
            arguments: ir_arguments,
        },
    );
}
