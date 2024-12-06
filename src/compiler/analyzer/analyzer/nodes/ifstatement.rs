use crate::compiler::{
    analyzer::{
        analyzer::{handle_body, handle_expression},
        FunctionCtx, ProgramCtx,
    },
    errors::Location,
    parser::{ExpressionInfo, NodeInfo},
    types::{BaseType, Type},
};

pub fn handle_ifstatement(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location,
    expression: ExpressionInfo,
    body: Vec<NodeInfo>,
    else_body: Option<Vec<NodeInfo>>,
) {
    let (result, _) = handle_expression(
        program,
        function,
        &Some(Type::new(BaseType::Boolean)),
        Some(expression),
    );

    let yes = function.variables.increment();
    let no = function.variables.increment();
    let exit = function.variables.increment();

    function.operations.branch(&result, &yes, &no);
    function.operations.label(&yes);
    function.operations.goto(&exit);

    function.operations.label(&no);
    match else_body {
        Some(body) => {
            handle_body(program, function, body);
        }
        None => {}
    }
    function.operations.goto(&exit);
    function.operations.label(&exit);
}
