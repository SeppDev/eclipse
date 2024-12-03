use crate::compiler::{
    analyzer::{IRFunction, IRProgram, Operation},
    string::BetterString,
};

pub fn codegen(program: IRProgram) -> String {
    let mut source = BetterString::new();
    source.pushln("target triple = \"x86_64-pc-windows-unkown\"\n");

    for (key, value) in program.static_strings {
        source.pushln(format!(
            "@{key} = private constant [ {} x i8 ] c\"{value}\\00\"",
            value.len() + 1
        ));
    }

    source.pushln(
        "declare i32 @printf(i8*, ...)
declare i32 @sleep(i32)

declare i32 @fflush(ptr)
@stdout = external global ptr
        
@format = private constant [4 x i8] c\"%d\\0A\\00\"
define void @print(i32 %x) {
start:
    %fmt_ptr = getelementptr [5 x i8], ptr @format, i32 0, i32 0
    call i32 @printf(i8* %fmt_ptr, i32 %x)

    %stdout_ptr = load ptr, ptr @stdout
    call i32 @fflush(ptr %stdout_ptr)

    ret void
}
",
    );

    for function in program.functions {
        handle_function(&mut source, function);
    }

    return source.to_string();
}

fn handle_function(source: &mut BetterString, function: IRFunction) {
    let data_type = &function.return_type;
    let name = &function.name;

    let parameters = function
        .parameters
        .into_iter()
        .map(|(key, data_type)| format!("{data_type} %{key}"))
        .collect::<Vec<String>>()
        .join(", ");
    source.pushln(format!("define {data_type} @{name}({parameters}) {{"));
    source.pushln("start:");

    let mut body = BetterString::new();

    for operation in function.operations {
        if !matches!(operation, Operation::Label(_)) {
            body.push("\t");
        }
        body.pushln(match operation {
            Operation::Label(label) => format!("{}:", label),
            Operation::Call {
                function,
                return_type,
                arguments,
            } => {
                format!("call {return_type} @{function}({arguments})")
            }
            Operation::StoreCall {
                destination,
                function,
                return_type,
                arguments,
            } => {
                format!("%{destination} = call {return_type} @{function}({arguments})")
            }
            Operation::Allocate {
                destination,
                data_type,
            } => {
                format!("%{destination} = alloca {data_type}")
            }
            Operation::Store {
                data_type,
                value,
                destination,
            } => {
                format!("store {data_type} {value}, ptr %{destination}")
            }
            Operation::Load {
                destination,
                destination_type,
                value,
            } => {
                format!("%{destination} = load {destination_type}, ptr {value}")
            }
            Operation::BinaryOperation {
                destination,
                operator,
                data_type,
                first,
                second,
            } => {
                format!(
                    "%{destination} = {} {data_type} {first}, {second}",
                    operator.convert(&data_type)
                )
            }
            Operation::CompareOperation {
                destination,
                operator,
                data_type,
                first,
                second,
            } => {
                format!(
                    "%{destination} = {} {data_type} {first}, {second}",
                    operator.convert(&data_type)
                )
            }
            Operation::Return { data_type, value } => format!("ret {} {}", data_type, value),
            Operation::Branch { condition, yes, no } => {
                format!("br i1 {condition}, label %{yes}, label %{no}")
            }
            Operation::Goto { label } => format!("br label %{label}"),
            Operation::Unkown => panic!(),
        });
    }

    source.push(body.to_string());
    source.pushln("}\n");
}
// fn handle_node(node: IRNode, body: &mut BetterString) {
//     match node {
//         Operation::Return(info) => {
//             let data_type = &info.data_type;
//             let value = handle_expression(body, &info);

//             body.pushln(format!("ret {data_type} {value}"));
//         },
//         _ => todo!()
//     }
// }

// fn handle_expression(body: &mut BetterString, info: &IRExpressionInfo) -> String {
//     let mut expression = BetterString::new();

//     // let data_type = &info.data_type;
//     match &info.expression {
//         IRExpression::Void => {},
//         IRExpression::Integer(int) => expression.push(int),
//         _ => todo!("{:?}", info)
//     }

//     return expression.to_string();
// }
