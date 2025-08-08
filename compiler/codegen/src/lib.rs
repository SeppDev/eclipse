use common::string::Appendable;
use context::CompilerCtx;
use syntax::mir::{Expression, Module, Node, Parameter, Type};

mod llvm;

pub struct Codegen {
    pub source: String,
}

pub fn generate(compiler: &CompilerCtx, module: Module) -> String {
    let mut codegen = Codegen {
        source: String::new(),
    };

    codegen
        .source
        .pushln("target triple = \"x86_64-unkown-linux-gnu\"");

    // for function in module.functions {}

    codegen.source
}

impl Codegen {
    pub fn type_to_string(data_type: &Type) -> String {
        match data_type {
            Type::Void => "void".to_string(),
            Type::Boolean => "i1".to_string(),
            Type::Bytes(bytes) => format!("[{bytes} x i8]"),
            Type::Int(bits) => format!("i{bits}"),
        }
    }
    pub fn write_type(&mut self, data_type: &Type) {
        let data_type = Codegen::type_to_string(data_type);
        self.source.push_string(data_type);
    }

    fn expression_to_string(&mut self, expression: Expression) -> String {
        use Expression::*;

        match expression {
            _ => todo!(),
        }
    }

    fn function(
        &mut self,
        name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Vec<Node>,
    ) {
        self.source.pushln(format!("define i32 @{name}() {{\n"));
        self.source.pushln("start:");

        body.into_iter().for_each(|n| self.node_to_string(n));

        self.source.pushln("}");
    }
    fn node_to_string(&mut self, node: Node) {
        use Node::*;

        match node {
            Return(expr) => match expr {
                Some(expr) => match expr {
                    Expression::Integer(data_type, int) => {
                        let data_type = Codegen::type_to_string(&data_type);
                        self.source.push_string(format!("ret {data_type} {int}"))
                    }
                },
                None => self.source.push_str("ret void"),
            },
            Allocate(data_type) => {
                let data_type = Codegen::type_to_string(&data_type);
                self.source.push_string(format!("alloca {data_type}"));
            }
            Goto(label) => self.source.push_string(format!("br %{label}")),
            Set {
                name,
                data_type,
                value,
            } => todo!(),
        };

        self.source.line();
    }
}
