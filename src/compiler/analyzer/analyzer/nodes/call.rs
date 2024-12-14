use crate::compiler::{
    analyzer::{analyzer::what_type, FunctionCtx, IRValue, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
    path::Path,
};

use super::allocate::handle_read;

pub fn handle_call(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location,
    path: Path,
    mut arguments: Vec<ExpressionInfo>,
) {
    let found = match program.types.get_function(function.relative_path, &path, program.namespaces) {
        Some(f) => f,
        None => {
            program
                .debug
                .error(location, format!("Could not find function: '{path}'"));
            return;
        }
    };

    if arguments.len() != found.parameters.len() {
        program.debug.error(
            location,
            format!(
                "Expected {} arguments, but got {}",
                found.parameters.len(),
                arguments.len()
            ),
        );
        return;
    }

    arguments.reverse();

    let mut ir_arguments = Vec::new();
    for param_type in &found.parameters {
        let info = arguments.pop().unwrap();

        let data_type = what_type(program, function, &info.location, Some(param_type), &info);
        let value = handle_read(program, function, &location, &data_type, info);
        
        ir_arguments.push((param_type.convert(), value));
    }

    function.operations.call(&found.key, &found.return_type.convert(), IRValue::Arguments(ir_arguments));
}
