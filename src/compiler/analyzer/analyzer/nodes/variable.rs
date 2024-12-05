use crate::compiler::{
    analyzer::{
        analyzer::{handle_expression, what_type},
        FunctionCtx, Operation, ProgramCtx,
    },
    errors::Location,
    parser::ExpressionInfo,
    types::Type,
};

use super::handle_allocation;

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
                    function.operations.push(Operation::Allocate {
                        destination: key,
                        data_type: dt.convert(),
                    });
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
        Some(dt) => dt,
        None => what_type(program, function, None, &info),
    };

    let destination = function
        .variables
        .insert(false, &name, mutable, data_type.clone(), location.clone())
        .key
        .clone();

    handle_allocation(program, function, &location, destination, data_type, info);
}

pub fn handle_set_variable(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location,
    name: String,
    expression: Option<ExpressionInfo>,
) {
    let variable = match function.variables.read(&name) {
        Some(var) => var.clone(),
        None => {
            program
                .debug
                .error(location, format!("Could not find variable named: '{name}'"));
            return;
        }
    };

    if !variable.mutable {
        program.debug.error(
            location,
            format!("Cannot mutate unmutable variable: '{name}'"),
        );
        return;
    }

    let (value, data_type) = handle_expression(
        program,
        function,
        &Some(variable.data_type.clone()),
        expression,
    );

    function.operations.push(Operation::Store {
        data_type: data_type.convert(),
        value,
        destination: variable.key.clone(),
    });
}
