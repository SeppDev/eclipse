use crate::compiler::{
    analyzer::{analyzer::{handle_body, program::ProgramCtx}, handle_expression, FunctionCtx},
    errors::Location,
    parser::{ExpressionInfo, NodeInfo},
    types::Type,
};

pub fn handle_ifstatement(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    return_type: &Option<Type>,
    location: Location,
    expression: ExpressionInfo,
    body: Vec<NodeInfo>,
    else_body: Option<Vec<NodeInfo>>,
) {
    let result = handle_expression(
        program,
        function,
        &location,
        None,
        &Type::boolean(),
        expression,
    );

    let yes = function.increment_key();
    let no = function.increment_key();
    let exit = function.increment_key();

    function.operations.branch(&result, &yes, &no);
    function.operations.label(&yes);
    handle_body(program, function, return_type, body);

    function.operations.goto(&exit);

    function.operations.label(&no);
    match else_body {
        Some(body) => {
            handle_body(program, function, return_type, body);
        }
        None => {}
    }
    function.operations.goto(&exit);
    function.operations.label(&exit);
}
