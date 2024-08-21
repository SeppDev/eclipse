use std::collections::HashMap;

use crate::parser::{Expression, Type, Value};

use super::{codegen::Program, writer::Writer};

#[derive(Debug, Clone)]
pub struct Variable {
    pub offset: usize,
    pub var_type: Type,
}

pub struct Function{
    variables: HashMap<String, Variable>,

    // program: &'a Program,
    pub stack_size: usize,
    pub writer: Writer,
}
impl Function {
    pub fn new() -> Self {
        Self {
            stack_size: 0,
            // program: &'a program,
            variables: HashMap::new(),
            writer: Writer::new(),
        }
    }
    pub fn get_variable(&mut self, name: &String) {}

    fn expression(&mut self, expression: Expression) -> String {
        return match expression {
            Expression::Value(value, typ) => match typ {
                Type::Integer(_) => match value {
                    Value::Integer(v) => format!("{}", v),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            Expression::GetVariable(name) => {
                let variable = self.variables.get(&name).unwrap();
                self.writer
                    .add_operation(&format!("mov rax, qword [rbp-{}]", variable.offset));
                format!("rax")
            }
            Expression::Call(name, arguments) => {
                // let function = self.program.
                todo!()
            },
            Expression::BinaryOperation(a, operator, b) => todo!(),
        };
    }
    pub fn define_variable(&mut self, name: &String, var_type: Type, expression: Expression) {
        let value = self.expression(expression);
        self.stack_size += 8;
        self.writer
            .add_operation(&format!("mov qword [rbp-{}], {}", self.stack_size, value));
        self.variables.insert(
            name.clone(),
            Variable {
                offset: self.stack_size,
                var_type: var_type,
            },
        );
    }
}
