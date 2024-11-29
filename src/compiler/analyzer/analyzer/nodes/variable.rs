use crate::compiler::{
    analyzer::{analyzer::handle_expression, FunctionCtx, Operation, ProgramCtx},
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
    let (value, data_type) = handle_expression(program, function, &data_type, expression);
    let t1 = data_type.convert();
    let t2 = data_type.convert();

    let variable = function
        .variables
        .insert(&name, mutable, data_type, location);

    function.operations.push(Operation::Allocate {
        destination: variable.key.clone(),
        data_type: t1,
    });

    function.operations.push(Operation::Store {
        data_type: t2,
        value,
        destination: variable.key.clone(),
    });
}
