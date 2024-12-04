use crate::compiler::{
    analyzer::{analyzer::{handle_body, handle_expression, LoopInfo}, FunctionCtx, Operation, ProgramCtx},
    errors::Location,
    parser::{ExpressionInfo, NodeInfo}, types::{BaseType, Type},
};

pub fn handle_loop(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    _location: Location,
    condition: Option<ExpressionInfo>,
    body: Vec<NodeInfo>,
) {
    let begin = function.variables.increment();
    let end = function.variables.increment();
    function.loop_info.push(LoopInfo::new(begin.clone(), end.clone()));

    function.operations.push(Operation::Goto { label: begin.clone() });
    function.operations.push(Operation::Label(begin.clone()));

    match condition {
        Some(expression) => {
            let (result, _) = handle_expression(
                program,
                function,
                &Some(Type::new(BaseType::Boolean)),
                Some(expression),
            );
            let after = function.variables.increment();
            function.operations.push(Operation::Branch { condition: result, yes: after.clone(), no: end.clone() });
            function.operations.push(Operation::Label(after))
        },
        None => {}
    }

    handle_body(program, function, body);

    function.operations.push(Operation::Goto { label: begin });
    function.operations.push(Operation::Label(end));

    let _ = function.loop_info.pop();
}

pub fn handle_break(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location
) {
    let last = match function.loop_info.last() {
        Some(li) => li,
        None => {
            program
                .debug
                .error(location, "Break can only be used inside of loops");
            return;
        }
    };
    function.operations.push(Operation::Goto {
        label: last.end.clone(),
    }); 
}

pub fn handle_continue(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location
) {
    let last = match function.loop_info.last() {
        Some(li) => li,
        None => {
            program
                .debug
                .error(location, "Continue can only be used inside of loops");
            return;
        }
    };
    function.operations.push(Operation::Goto {
        label: last.begin.clone(),
    });
}
