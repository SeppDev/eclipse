use crate::compiler::{
    analyzer::{analyzer::what_type, handle_expression, handle_read, handle_store, FunctionCtx, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
    types::{ReferenceState, Type},
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
                    let key = function.variables.increment();
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

    let destination = function
        .variables
        .insert(true, name, mutable, data_type.clone(), location.clone())
        .key
        .clone();

    handle_expression(program, function, &location, &destination, true, &data_type, info);
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

    let variable = match function.variables.read(&name, &ReferenceState::None) {
        Some(var) => var.clone(),
        None => {
            program.debug.error(
                location,
                format!("Cannot modify a variable that has not been declared: '{name}'"),
            );
            return;
        }
    };

    if !variable.mutable {
        let message = program.debug.error(
            variable.location.clone(),
            format!("Cannot mutate unmutable variable: '{name}'"),
        );
        message.set_notice(format!("help: mut {name}"));
        message.push("", location);
        return;
    }

    let value = handle_read(
        program,
        function,
        &location,
        &variable.data_type.clone(),
        expression,
    );

    function
        .operations
        .store(&variable.data_type.convert(), &value, &variable.key);
}
