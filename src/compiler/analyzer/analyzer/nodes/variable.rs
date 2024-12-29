use crate::compiler::{
    analyzer::{analyzer::what_type, handle_expression, FunctionCtx, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
    types::Type,
};

pub fn handle_variable_declaration(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location,
    name: String,
    mutable: bool,
    data_type: Option<Type>,
    expression: Option<ExpressionInfo>,
) {
    let info = match expression {
        Some(e) => e,
        None => {
            return match data_type {
                Some(dt) => {
                    let key = function.increment_key();
                    function.operations.allocate(&key, &dt.convert());
                }
                None => {
                    program
                        .debug
                        .error(location, format!("Type annotations needed"));
                }
            };
        }
    };

    let data_type = match data_type {
        Some(dt) => what_type(program, function, &info.location, Some(&dt), &info),
        None => what_type(program, function, &location, None, &info),
    };

    let destination = function.increment_key();
    function
        .operations
        .allocate(&destination, &data_type.convert());


    handle_expression(
        program,
        function,
        &location,
        Some(&destination),
        &data_type,
        info,
    );

    function.insert_variable(name, Some(destination), mutable, data_type.clone(), location.clone());
}

pub fn handle_set_variable(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location,
    name: String,
    expression: Option<ExpressionInfo>,
) {
    let expression = match expression {
        Some(e) => e,
        None => {
            program.debug.error(
                location,
                format!("Cannot set a variable without any expression"),
            );
            return;
        }
    };

    let variable = function.read_variable(&name).unwrap().clone();

    if !variable.mutable {
        let message = program.debug.error(
            variable.location.clone(),
            format!("Cannot mutate unmutable variable: '{name}'"),
        );
        message.set_notice(format!("help: mut {name}"));
        message.push("", location);
        return;
    }

    handle_expression(
        program,
        function,
        &location,
        Some(&variable.key),
        &variable.data_type,
        expression,
    );
}
