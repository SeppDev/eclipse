use std::path::PathBuf;

use crate::{
    builder::{function::Function, labels::Labels, writer::Writer},
    parser::{analyze, parse, Node},
    FILE_EXTENSION,
};
use eclipse::{execute, CompileError};
use scope::scope;

mod scope;

pub struct Program {
    labels: Labels,
}
impl Program {
    pub fn new() -> Self {
        Self {
            labels: Labels::new(),
        }
    }
}

pub fn compile(project_path: PathBuf) -> Result<String, CompileError> {
    let name = "app";
    let path = PathBuf::from(&project_path).join(format!("src/main.{}", FILE_EXTENSION));
    let nodes = match parse(&path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let nodes = match analyze(nodes, path, true) {
        Ok(nodes) => nodes,
        Err(error) => return Err(error),
    };

    // println!("{:#?}", nodes);

    let mut program = Program::new();
    let mut writer = Writer::new();
    writer.push_str("bits 64\n");
    writer.push_str("global main\n");
    writer.push_str("extern puts, exit\n\n");
    writer.push_str("section .text\n");

    for node in nodes {
        match node {
            #[allow(unused)]
            Node::Function {
                public,
                name,
                parameters,
                return_type,
                body,
            } => {
                writer.label(&format!(".{}", program.labels.generate(&name)));

                let mut function = Function::new();
                writer.add_operation_str("push rbp");
                writer.add_operation_str("mov rbp, rsp");

                scope(body, &mut function, &mut program);

                writer.add_operation(&format!("sub rsp, {}", function.stack_size));
                writer.writer(function.writer);
                writer.add_operation(&format!("add rsp, {}", function.stack_size));
                writer.add_operation_str("leave");
                writer.add_operation_str("ret");
            }
            _ => panic!(),
        }
    }

    writer.push_str("main:\n");
    writer.add_operation_str("push rbp");
    writer.add_operation_str("mov rbp, rsp");
    writer.add_operation_str("sub rsp, 16");

    let main_label = program.labels.get(&String::from("main"));
    writer.add_operation(&format!("call .{}", main_label));
    writer.add_operation_str("mov rcx, 0");
    writer.add_operation_str("call exit");

    let build_path = project_path.join("build");
    std::fs::create_dir(&build_path).unwrap_or_default();

    let assembly_file = build_path.clone().join("app.s");
    match std::fs::write(&assembly_file, writer.body) {
        Ok(()) => {}
        Err(error) => return Err(CompileError::OpenFile(error)),
    };

    match execute(format!(
        "nasm -f win64 {}",
        String::from(assembly_file.to_str().unwrap())
    )) {
        Ok(_out) => {}
        Err(error) => return Err(CompileError::NASM(error)),
    }

    let object_file = String::from(build_path.join(format!("{}.obj", &name)).to_str().unwrap());
    let executable = String::from(build_path.join(format!("{}", &name)).to_str().unwrap());

    match execute(format!("gcc -o {}.exe {} -m64", executable, object_file)) {
        Ok(_out) => {}
        Err(error) => return Err(CompileError::GCC(error)),
    }

    return Ok(executable);
}
