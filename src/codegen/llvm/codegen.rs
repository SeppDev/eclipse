use crate::{
    analyzer::{IRExpression, IRModule, IRNode, IRProgram},
    codegen::{
        builder::{self, Builder},
        llvm::expressions::extract_value, string::BetterString,
    },
    types::{Type, Value},
};

pub fn generate(program: IRProgram, builder: &mut Builder) {
    for module in program.modules {
        handle_module(builder, module);
    }
}

fn handle_module(builder: &mut Builder, module: IRModule) {
    for function in module.functions {
        builder.pushln(format!(
            "define {} @{}() {{\nentry:\n",
            convert_type(&function.return_type),
            function.name
        ));

        handle_scope(builder, function.nodes, &function.return_type, None);
        builder.pushln(format!("}}\n"));
    }
}

fn handle_scope(
    builder: &mut Builder,
    nodes: Vec<IRNode>,
    return_type: &Type,
    break_label: Option<&String>,
) {
    use IRNode::{self};

    for node in nodes {
        match node {
            IRNode::Loop(body) => {
                let break_label = builder.generate();
                let loop_label = builder.generate();
                
                builder.pushln(format!("\tbr label %{}", loop_label));
                builder.pushln(format!("{}:", loop_label));

                handle_scope(builder, body, return_type, Some(&break_label));

                builder.pushln(format!("\tbr label %{}", loop_label));
                builder.pushln(format!("{}:", break_label));
            },
            IRNode::Break => {
                builder.pushln(format!("\tbr label %{}", break_label.unwrap()));
            }
            IRNode::Return(expression) => match expression {
                Some(expression) => {
                    let str_type = convert_type(&return_type);

                    match expression {
                        IRExpression::Value(value) => {
                            builder.pushln(format!("\tret {} {}", str_type, extract_value(value)));
                        }
                        IRExpression::GetVariable(name) => {
                            let result_name = builder.generate();
                            builder.pushln(format!(
                                "\t%{} = load {}, {}* %{}",
                                result_name, str_type, str_type, name
                            ));
                            builder.pushln(format!("\tret {} %{}", str_type, result_name));
                        }
                        _ => todo!(),
                    }
                }
                None => builder.pushln("\tret void"),
            },
            IRNode::DefineVariable(name, var_type, expression) => {
                let str_type = convert_type(&var_type);
                // builder.pushln(format!("\t%{}")); add type value, 0

                builder.pushln(format!(
                    "\t%{} = alloca {}, align {}",
                    name,
                    str_type,
                    var_type.size()
                ));
                builder.push(format!("\tstore {} ", str_type));

                match expression {
                    IRExpression::Value(value) => builder.push(extract_value(value)),
                    _ => todo!(),
                }

                builder.pushln(format!(", {}* %{}", str_type, name));
            }
            IRNode::Call(name, data_type, arguments) => {
                let string = call(builder, name, &data_type, arguments);
                builder.push('\t');
                builder.pushln(string);
            },
            _ => todo!(),
        }
    }
}

// fn clone()

fn call(builder: &mut Builder, name: String, data_type: &Type, arguments: Vec<(IRExpression, Type)>) -> String {
    let str_type = convert_type(data_type);

    let body = format!("call {} @{}({})", str_type, name, {
        let mut body = BetterString::new();
        for (argument, t) in arguments {
            let str_type = convert_type(&t);

            let value = match argument {
                IRExpression::Value(value) => extract_value(value),
                IRExpression::GetVariable(name) => {
                    let result_name = builder.generate();

                    builder.pushln(format!(
                        "\t%{} = load {}, {}* %{}",
                        result_name, str_type, str_type, name
                    ));

                    format!("%{}", result_name)
                }
                expr => todo!("{:?}", expr)
            };

            body.push(format!("{} {} ", str_type, value));
        }

        body.to_string()
    });

    body
}

fn convert_type(data_type: &Type) -> String {
    use crate::types::BaseType;

    return match data_type {
        Type::Base(base) => match base {
            BaseType::Boolean => "i1",
            BaseType::Int8 => "i8",
            BaseType::Int16 => "i16",
            BaseType::Int32 => "i32",
            BaseType::Int64 => "i64",
            // BaseType::Float16 => "half",
            BaseType::Float32 => "float",
            BaseType::Float64 => "double",
            // BaseType::Float128 => "fp128",
            BaseType::Void => "void",
            _ => todo!(),
        }
        .to_string(),
        t => todo!("{:?}", t),
    };
}

// fn handle_scope(builder: &mut Builder, nodes: Vec<IRNode>, return_type: &Type) {
//     for node in nodes {
//         match node {
//             IRNode::Loop(nodes) => {
//                 let break_label = builder.random.generate();
//                 let loop_label = builder.random.generate();

//                 builder.pushln(format!("br label %{}", loop_label));
//                 builder.pushln(format!("{}:", loop_label));
//                 handle_scope(builder, nodes, return_type);

//                 builder.pushln(format!("\tbr label %{}", loop_label));
//                 builder.pushln(format!("{}:", break_label));
//             },
//             IRNode::DefineVariable(name, data_type, expression) => {
//                 let expression = handle_expression(builder, expression, &data_type, Some(&name));
//                 builder.push(format!("\t%{} = {}", name, expression));
//             }
//             IRNode::Return(expression) => {
//                 let expression = match expression {
//                     Some(expr) => expr,
//                     None => {
//                         builder.pushln_str("\tret void");
//                         continue;
//                     }
//                 };

//                 let result_name = builder.random.generate();

//                 let expression =
//                     handle_expression(builder, expression, &return_type, Some(&result_name));

//                 builder.pushln(format!("\t%{} = {}", result_name, expression));
//                 builder.pushln(format!("\tret {} %{}", convert_type(&return_type), result_name));
//             }
//             // IRNode::Call(name, data_type, arguments) => {
//             //     let expression = handle_expression(builder, expression, &data_type, None);
//             //     builder.push(format!("{}", expression));

//             // }
//             IRNode::SetVariable(name, data_type, expression) => {
//                 let expr = handle_expression(builder, expression, &data_type, None);
//                 builder.pushln(format!("\tstore {}, {}* %{}", expr, convert_type(&data_type), name));
//             },
//             _ => todo!()
//         }
//         builder.next_line();
//     }
// }

// fn handle_expression(
//     builder: &mut Builder,
//     expression: IRExpression,
//     var_type: &Type,
//     declaration: Option<&String>,
// ) -> BetterString {
//     use crate::parser::Value;
//     let expr_type = convert_type(var_type);
//     let mut string = BetterString::new();

//     match expression {
//         IRExpression::BinaryOperation(expr1, operator, expr2) => {
//             let result = builder.random.generate();
//             let a = *expr1;
//             let b = *expr2;

//             // let first = handle_expression(builder, a, var_type, );
//             // let second = handle_expression(builder, b, var_type, None);
//             // string.push_str(format!("%{} = add {} {}, {}", result, convert_type(var_type), first, second).as_str());
//         }
//         IRExpression::Value(value) => match value {
//             Value::Integer(_, value) => match declaration {
//                 Some(name) => {
//                     string.push(format!("alloca {}\n", expr_type).as_str());
//                     string.push(
//                         format!("\tstore {} {}, {}* %{}", expr_type, value, expr_type, name)
//                             .as_str(),
//                     );
//                 }
//                 None => {
//                     string.push(format!("{} {}", expr_type, value).as_str());
//                 }
//             },
//             _ => todo!(),
//         },
//         IRExpression::GetVariable(name) => match declaration {
//             Some(_) => {
//                 string.push(format!("load {}, {}* %{}", expr_type, expr_type, name).as_str());
//             }
//             None => {
//                 string.push(format!("{} %{}", expr_type, name).as_str());
//             }
//         },
//         IRExpression::Call(name, arguments) => {
//             let mut args: Vec<String> = Vec::new();

//             for (argument, data_type) in arguments {
//                 let result_name = builder.random.generate();
//                 let s_type = convert_type(&data_type);

//                 let expression =
//                     handle_expression(builder, argument, &data_type, Some(&result_name));
//                 builder.pushln(format!("\t%{} = {}", result_name, expression));
//                 args.push(format!("{} %{}", s_type, result_name));
//             }

//             string
//                 .push(format!("\tcall {} @{}({})", expr_type, name, args.join(", ")).as_str());
//         }
//     };

//     return string;
// }
