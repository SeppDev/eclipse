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
            IRNode::DefineVariable {
                name,
                data_type,
                expression,
            } => {
                builder.push(format!("\t%{} = ", name));
                define_variable(builder, expression, &data_type);
            }
            IRNode::Return(expression) => {
                let expression = match expression {
                    Some(expr) => expr,
                    None => {
                        builder.pushln_str("\tret void");
                        continue;
                    }
                };

                builder.push_str("\tret ");
                handle_return(builder, expression, return_type);
            }
        }
        builder.next_line();
    }
}

fn define_variable(builder: &mut Builder, expression: IRExpression, var_type: &Type) {
    use crate::parser::Value;

    let expr_type = convert_type(var_type);

    match expression {
        IRExpression::Value(value) => match value {
            Value::Integer(signed, value) => {
                builder.push(format!("add {} {}, 0", expr_type, value))
            }
            Value::Float(value) => builder.push(format!("fadd {} {}, 0.0", expr_type, value)),
            _ => {}
        },
        IRExpression::Call(name, arguments) => builder.push(format!(
            "call {} @{}()",
            expr_type,
            name
        )),
        _ => todo!(),
    }
}

fn handle_return(builder: &mut Builder, expression: IRExpression, return_type: &Type) {
    use crate::parser::Value;

    let expr_type = convert_type(return_type);

    match expression {
        IRExpression::Value(value) => match value {
            Value::Integer(_signed, value) => builder.push(format!("{} {}", expr_type, value)),
            Value::Float(value) => builder.push(format!("{} {}", expr_type, value)),
            _ => {}
        },
        IRExpression::GetVariable(name) => builder.push(format!("{} %{}", expr_type, name)),
        _ => todo!(),
    }
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
