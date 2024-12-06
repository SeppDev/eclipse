use crate::compiler::{
    analyzer::{
        analyzer::{handle_expression, what_type},
        variables::Variable,
        ElemmentPointerOperation, FunctionCtx, IRValue, Operation, ProgramCtx,
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

    match info.expression {
        Expression::Value(value) => {
            function.operations.push(Operation::Allocate {
                destination: destination.clone(),
                data_type: data_type.convert(),
            });

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
        Expression::Index(_, _) => {
            handle_read(
                program,
                function,
                location,
                destination.clone(),
                data_type.clone(),
                info,
            );
        }
        Expression::GetVariable(path) => {
            function.operations.push(Operation::Allocate {
                destination: destination.clone(),
                data_type: data_type.convert(),
            });

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
            function.operations.push(Operation::Allocate {
                destination: destination.clone(),
                data_type: data_type.convert(),
            });

            let (inner_type, size) = data_type.clone().array_info();

            if items.len() != size {
                let length = items.len();
                program.debug.error(
                    info.location.clone(),
                    format!("Expected {length} items but found {size} items"),
                );
            }

            for (index, item) in items.into_iter().enumerate() {
                let key_ptr = function.variables.increment();

                let operation = ElemmentPointerOperation::Inbounds {
                    data_type: data_type.convert(),
                    value_type: inner_type.convert(),
                    from: destination.clone(),
                    index: IRValue::IntLiteral(format!("{index}")),
                };

                function.operations.push(Operation::GetElementPointer {
                    destination: key_ptr.clone(),
                    operation,
                });

                handle_store(
                    program,
                    function,
                    location,
                    key_ptr,
                    inner_type.clone(),
                    item,
                );
            }
        }
        _ => todo!("{:#?}", info),
    };
}

pub fn handle_store(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    destination: String,
    data_type: Type,
    info: ExpressionInfo,
) {
    return match info.expression {
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
        },
        _ => todo!()
    }
}

pub fn handle_read(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    destination: String,
    data_type: Type,
    info: ExpressionInfo,
) -> IRValue {
    return match info.expression {
        Expression::Index(path, index) => {
            let name = path.components().pop().unwrap();
            let info = *index;

            let array_value_ptr = function.variables.increment();
            let result_ptr = function.variables.increment();

            let array = match function.variables.read(&name) {
                Some(var) => var.clone(),
                None => {
                    program.debug.error(
                        info.location.clone(),
                        format!("Could not find variable named: '{name}'"),
                    );
                    return IRValue::Null;
                }
            };

            let (inner_type, _) = array.data_type.clone().array_info();
            // let array_type = data_type.convert();
            let value = handle_read(program, function, location, destination, data_type, info);

            function.operations.push(Operation::GetElementPointer {
                destination: array_value_ptr.clone(),
                operation: ElemmentPointerOperation::Inbounds {
                    data_type: array.data_type.convert(),
                    value_type: inner_type.convert(),
                    from: array.key,
                    index: value.clone(),
                },
            });

            function.operations.push(Operation::Load {
                destination: result_ptr.clone(),
                destination_type: inner_type.convert(),
                value: IRValue::Variable(array_value_ptr),
            });

            IRValue::Variable(result_ptr)
        }
        Expression::GetVariable(path) => {
            let name = path.components().pop().unwrap();
            IRValue::Variable(name)
        }
        Expression::Value(value) => match value {
            Value::Integer(int) => IRValue::IntLiteral(int),
            Value::Boolean(bool) => IRValue::BoolLiteral(bool),
            Value::Float(float) => IRValue::FloatLiteral(float),
            _ => todo!(),
        },

        // Expression::Value(value) if data_type.base.is_integer() => ,
        // Expression::Value(_) => {
        //     program.debug.error(
        //         info.location.clone(),
        //         format!("An integer is required to index in an array"),
        //     );
        //     IRValue::Null
        // }
        _ => todo!(),
    };
    /*
        let name = path.components().pop().unwrap();
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
    }); */
}
