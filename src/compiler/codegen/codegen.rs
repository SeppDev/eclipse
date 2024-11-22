use crate::compiler::{
    analyzer::{IRFunction, IRNode, IRProgram},
    string::BetterString, types::{BaseType, Type},
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
    let data_type = convert_type(&function.return_type);
    let name = &function.name;
    source.pushln(format!("define {data_type} @{name}() {{"));
    source.pushln("entry:");

    let mut body = BetterString::new();

    for node in function.body {
        handle_node(node, &mut body);
    }
    
    source.push(body.to_string());
    source.pushln("}");
}

fn handle_node(node: IRNode,  body: &mut BetterString) {
    match node {
        IRNode::Return(info) => {
            let data_type = convert_type(&info.data_type);
            let value = 0; // info.expression
            body.pushln(format!("ret {data_type} {value}"));
        },
        _ => todo!()
    }  
}

fn convert_type(data_type: &Type) -> String {
    match data_type {
        Type::Base(base) => match base {
            BaseType::Void => "void",
            BaseType::Boolean => "i1",
            BaseType::Int32 => "i32", 
            _ => todo!()
        }
        _ => todo!()
    }.to_string()
}
