use std::{io::Write, path::PathBuf};

use crate::{
    parser::{self, node::{Expression, Node}},
    FILE_EXTENSION,
};
use code::Code;
use eclipse::{self, execute, BuildError};
use label::Label;
use variables::Variables;
mod code;
mod label;
mod variables;

struct Scope {
    content: Code,
    stack_size: usize,
    variables: Variables,
}

struct Builder {
    // externs: HashMap<String, bool>,
    project_path: String,
    body: Code,
    labels: Label,
}
impl Builder {
    fn new(project_path: String) -> Self {
        Self {
            // externs: HashMap::new(),
            project_path,
            body: Code::new(),
            labels: Label::new(),
        }
    }

    fn scope(&mut self, nodes: Vec<Node>, scope: &mut Scope) {
        let mut scope_variables: Vec<String> = Vec::new();

        for node in nodes {
            match node {
                Node::Scope(body) => self.scope(body, scope),
                Node::Conditional((a, b), body, else_body) => {
                    match a {
                        Expression::Value(_value) => todo!(),
                        Expression::GetVariable(name) => {
                            scope.content.add_operation(format!(
                                "mov rax, [rbp-{}]",
                                scope.variables.get(&name)
                            ));
                        }
                    };
                    match b {
                        Expression::Value(_value) => todo!(),
                        Expression::GetVariable(name) => {
                            scope.content.add_operation(format!(
                                "mov rbx, [rbp-{}]",
                                scope.variables.get(&name)
                            ));
                        }
                    };

                    scope.content.add_operation_str("cmp rax, rbx");
                    let end_label = self.labels.increment();

                    match else_body {
                        None => {
                            scope.content.add_operation(format!("jne {}", end_label));
                            self.scope(body, scope);
                        }
                        Some(else_body) => {
                            let else_label = self.labels.increment();
                            scope.content.add_operation(format!("jne {}", else_label));
                            self.scope(body, scope);

                            scope.content.add_operation(format!("jmp {}", end_label));
                            scope.content.label(else_label);
                            self.scope(else_body, scope);
                        }
                    }

                    scope.content.label(end_label);
                }
                Node::Call(name, arguments) => {
                    let mut argument_offset = 0;
                    for argument in arguments {
                        argument_offset += 8;

                        match argument {
                            Expression::Value(_value) => todo!(),
                            Expression::GetVariable(name) => {
                                scope.content.add_operation(format!(
                                    "mov rax, [rbp-{}]",
                                    scope.variables.get(&name)
                                ));
                            }
                        };

                        scope
                            .content
                            .add_operation(format!("mov [r8+{}], rax", argument_offset));
                    }

                    if name == "print" {
                        scope.content.add_operation_str("call print");
                    } else {
                        scope
                            .content
                            .add_operation(format!("call {}", self.labels.get(name).unwrap()));
                    }
                }
                Node::DefineVariable {
                    name,
                    mutable,
                    var_type,
                    expression,
                } => {
                    let variable = scope.variables.create(&name, &var_type);
                    scope_variables.push(name);
                    scope.stack_size += variable.size;

                    match expression.unwrap() {
                        Expression::Value(value) => {
                            scope.content.add_operation(format!(
                                "mov qword [rbp-{}], {}",
                                variable.offset, value
                            ));
                        }
                        Expression::GetVariable(name) => {
                            scope.content.add_operation(format!(
                                "mov rax, [rbp-{}]",
                                scope.variables.get(&name)
                            ));
                            scope
                                .content
                                .add_operation(format!("mov [rbp-{}], rax", variable.offset));
                        }
                    };
                }
                _ => todo!(),
            }
        }

        for key in scope_variables {
            scope.variables.remove(&key)
        }
    }
    pub fn build(&mut self, name: String) -> Result<String, BuildError> {
        self.body
            .push_str("global main\nextern exit, printf\nsection .text\n");

        let path = PathBuf::from(&self.project_path).join(format!("src/main.{}", FILE_EXTENSION));
        let source = match eclipse::read_file(path.to_str().unwrap()) {
            Ok(source) => source,
            Err(error) => return Err(error),
        };
        let nodes = match parser::parser::parse(source) {
            Ok(path) => path,
            Err(_) => return Err(BuildError::Parsing),
        };

        // panic!("{:#?}", nodes);

        for node in nodes {
            match node {
                Node::Function {
                    name,
                    parameters,
                    return_types,
                    body,
                } => {
                    let mut scope = Scope {
                        stack_size: 0,
                        variables: Variables::new(),
                        content: Code::new(),
                    };
                    let name = self.labels.generate(&name);

                    let mut param_offset = 0;
                    for (name, par_type) in parameters {
                        let size = 8;
                        let variable = scope.variables.create(&name, &par_type);
                        param_offset += size;

                        scope
                            .content
                            .add_operation(format!("mov r9, [r8+{}]", param_offset));
                        scope
                            .content
                            .add_operation(format!("mov [rbp-{}], r9", variable.offset));
                    }

                    self.scope(body, &mut scope);
                    self.body
                        .function(&name, scope.content.body, scope.stack_size)
                }
                node => return Err(BuildError::Building),
            }
        }

        // Generate assembly
        self.body.push_str(
            "print:
	push rbp
	mov rbp, rsp
	sub rsp, 64
	mov r9, [r8+8]
	mov qword [rbp-8], r9
    mov rax, qword [rbp-8]
    mov rdx, rax
    mov byte [rbp-13], 0
    mov byte [rbp-14], 10
    mov byte [rbp-15], 100
    mov byte [rbp-16], 37
    lea rcx, byte [rbp-16]
    call printf
	add rsp, 64
	mov rsp, rbp
	pop rbp
	ret\n\n",
        );

        let main_label = match self.labels.get(String::from("main")) {
            Some(key) => key,
            None => return Err(BuildError::Building),
        };

        let content = format!("\tcall {}\n\tmov rcx, 0\n\tcall exit\n", main_label);
        self.body.function(&String::from("main"), content, 16);

        let build_path = PathBuf::from(&self.project_path).join("build");
        std::fs::create_dir(&build_path).unwrap_or_default();

        let assembly_file = build_path.join(format!("{}.asm", &name));
        let mut file = std::fs::File::create(&assembly_file).unwrap();
        file.write(&self.body.body.as_bytes()).unwrap();

        match execute(format!(
            "nasm -f win64 {}",
            String::from(assembly_file.to_str().unwrap())
        )) {
            Ok(_out) => {}
            Err(error) => return Err(BuildError::NASM(error)),
        }

        let object_file = String::from(build_path.join(format!("{}.obj", &name)).to_str().unwrap());
        let executable = String::from(build_path.join(format!("{}", &name)).to_str().unwrap());

        match execute(format!("gcc -o {}.exe {} -m64", executable, object_file)) {
            Ok(_out) => {}
            Err(error) => return Err(BuildError::GCC(error)),
        }
        return Ok(executable);
    }
}

pub fn build(project_path: String, name: String) -> Result<String, BuildError> {
    let mut builder = Builder::new(project_path);
    builder.build(name)
}
