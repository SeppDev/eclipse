use crate::compiler::{
    analyzer::{
        analyzer::{handle_expression, what_type}, variables::Variable, ElemmentPointerOperation, FunctionCtx, IRValue, Operation, ProgramCtx
    },
    errors::Location,
    parser::{Expression, ExpressionInfo, Value},
    types::Type,
};

pub fn handle_allocation(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    destination: String,
    data_type: Type,
    info: ExpressionInfo,
) {
    function.operations.push(Operation::Allocate {
        destination: destination.clone(),
        data_type: data_type.convert(),
    });

    match info.expression {
        Expression::Value(value) => {
            let value = match value {
                Value::Integer(int) => IRValue::IntLiteral(int),
                Value::Boolean(bool) => IRValue::BoolLiteral(bool),
                Value::Float(float) => IRValue::FloatLiteral(float),
                _ => todo!(),
            };
            function.operations.push(Operation::Store {
                data_type: data_type.convert(),
                destination: destination.clone(),
                value,
            });
        }
        Expression::GetVariable(path) => {
            let name = path.first().unwrap();

            let result_ptr = match function.variables.read(name) {
                Some(var) => var.key.clone(),
                None => todo!(),
            };

            let result_key = function.variables.increment();

            function.operations.push(Operation::Load {
                destination: result_key.clone(),
                destination_type: data_type.convert(),
                value: IRValue::Variable(result_ptr.clone()),
            });

            IRValue::Variable(result_key);
        }
        Expression::Array(items) => {
            let (inner_type, size) = data_type.clone().array_info();
            // Compare given array length with size

            for (index, item) in items.into_iter().enumerate() {
                let key_ptr = function.variables.increment();

                let operation = ElemmentPointerOperation::Inbounds {
                    data_type: data_type.convert(),
                    value_type: inner_type.convert(),
                    from: destination.clone(),
                    index,
                };

                function.operations.push(Operation::GetElementPointer {
                    destination: key_ptr.clone(),
                    operation,
                });

                function.operations.push(Operation::Store {
                    data_type: inner_type.convert(),
                    value: IRValue::IntLiteral(format!("{index}")),
                    destination: key_ptr,
                });
            }
        }
        _ => todo!(),
    };
}
