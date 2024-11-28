use crate::compiler::{
    analyzer::{FunctionCtx, IRType, Operation, ProgramCtx},
    parser::ExpressionInfo,
    types::Type,
};

pub fn variable_declaration(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    name: String,
    mutable: bool,
    data_type: Option<Type>,
    expression: Option<ExpressionInfo>,
) {
    // let variable = function.variables.insert(&name, mutable, data_type, location);

    function.operations.push(Operation::Allocate {
        destination: String::from("my_var"),
        data_type: IRType::Integer(32),
    });
}
