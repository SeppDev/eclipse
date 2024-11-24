use crate::compiler::{
    analyzer::{IRFunction, IRProgram}, string::BetterString
};

pub fn codegen(program: IRProgram) -> String  {
    let mut source = BetterString::new();
    source.pushln("target triple = \"x86_64-pc-windows-unkown\"\n");

    for function in program.functions {
        handle_function(&mut source, function);
    }

    return source.to_string();
}

fn handle_function(source: &mut BetterString, function: IRFunction) {
    let data_type = &function.return_type;
    let name = &function.name;
    source.pushln(format!("define {data_type} @{name}() {{"));
    source.pushln("entry:");

    let body = BetterString::new();

    // for node in function.body {
    //     handle_node(node, &mut body);
    // }
    
    source.push(body.to_string());
    source.pushln("}");
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
