use crate::{
    analyzer::{IRExpression, IRModule, IRNode, IRProgram},
    codegen::builder::Builder,
    Type,
};

pub fn generate(program: IRProgram, builder: &mut Builder) {
    for module in program.modules {
        handle_module(builder, module);
    }
}

fn handle_module(builder: &mut Builder, module: IRModule) {
    for function in module.functions {
        builder.pushln(format!(
            "define {} @{}() local_unnamed_addr #0 {{\nentry:\n",
            convert_type(&function.return_type),
            function.name
        ));

        handle_scope(builder, function.nodes, &function.return_type);
        builder.pushln(format!("}}\n"));
    }
}

fn handle_scope(builder: &mut Builder, nodes: Vec<IRNode>, return_type: &Type) {
    for node in nodes {
        match node {
            IRNode::DefineVariable(name, data_type, expression) => {
                let expression = handle_expression(builder, expression, &data_type, Some(&name));
                builder.push(format!("\t%{} = {}", name, expression));
            }
            IRNode::Return(expression) => {
                let expression = match expression {
                    Some(expr) => expr,
                    None => {
                        builder.pushln_str("\tret void");
                        continue;
                    }
                };

                let result_name = builder.random.generate();

                let expression =
                    handle_expression(builder, expression, &return_type, Some(&result_name));

                builder.pushln(format!("\t%{} = {}", result_name, expression));
                builder.pushln(format!("\tret {} %{}", convert_type(&return_type), result_name));
            }
            IRNode::Expression(expression, data_type) => {
                let expression = handle_expression(builder, expression, &data_type, None);
                builder.push(format!("{}", expression));
            }
            IRNode::SetVariable(name, data_type, expression) => {
                let expr = handle_expression(builder, expression, &data_type, None);
                builder.pushln(format!("\tstore {}, {}* %{}", expr, convert_type(&data_type), name));
            }
        }
        builder.next_line();
    }
}

fn handle_expression(
    builder: &mut Builder,
    expression: IRExpression,
    var_type: &Type,
    declaration: Option<&String>,
) -> String {
    use crate::parser::Value;
    let expr_type = convert_type(var_type);
    let mut string = String::new();

    match expression {
        IRExpression::Value(value) => match value {
            Value::Integer(_, value) => match declaration {
                Some(name) => {
                    string.push_str(format!("alloca {}\n", expr_type).as_str());
                    string.push_str(
                        format!("\tstore {} {}, {}* %{}", expr_type, value, expr_type, name)
                            .as_str(),
                    );
                }
                None => {
                    string.push_str(format!("{} {}", expr_type, value).as_str());
                }
            },
            _ => todo!(),
        },
        IRExpression::GetVariable(name) => match declaration {
            Some(_) => {
                string.push_str(format!("load {}, {}* %{}", expr_type, expr_type, name).as_str());
            }
            None => {
                string.push_str(format!("{} %{}", expr_type, name).as_str());
            }
        },
        IRExpression::Call(name, arguments) => {
            let mut args: Vec<String> = Vec::new();

            for (argument, data_type) in arguments {
                let result_name = builder.random.generate();
                let s_type = convert_type(&data_type);

                let expression =
                    handle_expression(builder, argument, &data_type, Some(&result_name));
                builder.pushln(format!("\t%{} = {}", result_name, expression));
                args.push(format!("{} %{}", s_type, result_name));
            }

            string
                .push_str(format!("\tcall {} @{}({})", expr_type, name, args.join(", ")).as_str());
        }
    };

    return string;
}

fn convert_type(data_type: &Type) -> String {
    use crate::parser::BaseType;

    return match data_type {
        Type::Base(base) => match base {
            BaseType::Int64 => "i64",
            BaseType::Int32 => "i32",
            BaseType::Int16 => "i16",
            BaseType::Int8 => "i8",
            BaseType::Float16 => "half",
            BaseType::Float32 => "float",
            BaseType::Float64 => "double",
            BaseType::Float128 => "fp128",
            BaseType::Void => "void",
            _ => todo!(),
        }
        .to_string(),
        t => todo!("{:?}", t),
    };
}
