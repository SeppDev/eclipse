use crate::compiler::{
    analyzer::{FunctionCtx, IRType, IRValue, ProgramCtx},
    errors::Location,
    parser::{Expression, ExpressionInfo},
    types::Type, POINTER_WITH,
};

use super::handle_expression;

// pub fn handle_store(
//     program: &mut ProgramCtx,
//     function: &mut FunctionCtx,
//     location: &Location,
//     destination: &String,
//     data_type: &Type,
//     info: ExpressionInfo,
// ) {
//     let value = handle_expression(
//         program,
//         function,
//         location,
//         destination,
//         data_type,
//         info,
//     );
//     if data_type.base.is_basic() {
//         function
//             .operations
//             .store(&data_type.convert(), &value, &destination);
//     }
// }

pub fn handle_array_store(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    items: Vec<ExpressionInfo>,
    destination: &String,
    data_type: &Type,
    offset: usize,
) {
    let (item_type, _) = data_type.array_info();
    let item_size = item_type.bytes();

    if item_type.base.is_array() {
        for (index, item) in items.into_iter().enumerate() {
            let items = match item.expression {
                Expression::Array(items) => items,
                _ => panic!("Expected array"),
            };

            handle_array_store(
                program,
                function,
                location,
                items,
                destination,
                item_type,
                offset + index * item_size,
            )
        }
        return;
    }

    for (index, item) in items.into_iter().enumerate() {
        let key_ptr = function.variables.increment();

        function.operations.getelementptr_inbounds(
            &key_ptr,
            &data_type.convert(),
            destination,
            &IRType::Integer(POINTER_WITH),
            &IRValue::IntLiteral(format!("{}", index * item_size + offset)),
        );

        handle_expression(program, function, location, Some(key_ptr), item_type, item);
    }
}
