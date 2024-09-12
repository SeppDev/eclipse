use std::path::PathBuf;

use crate::{
    codegen::{IRNode, IRProgram},
    assembler::writer::Writer,
    execute, CompileError,
};

const NAME: &str = "app";

pub fn assemble(program: IRProgram, build_path: PathBuf) -> Result<PathBuf, CompileError> {
    let mut writer = Writer::new();

    println!("{:#?}", program);

    for (path, function) in program.body {
        writer.label(&path);
        writer.add_operation_str("push rbp");
        writer.add_operation_str("mov rbp, rsp");
        writer.add_operation(format!("sub rsp, {}", function.stack_size.max(16)));

        for node in function.body {
            match node {
                IRNode::StoreValue { offset, size, value } => {
                    writer.add_operation(format!("mov {:?} [rsp-{}], {}", size, offset, value))
                }
            }
        }

        writer.add_operation(format!("add rsp, {}", function.stack_size.max(16)));
        writer.add_operation_str("leave");
        writer.add_operation_str("ret");
    }

    /*    push rbp
    mov rbp, rsp
    sub rsp, 32 */

    writer.push_str("main:\n");
    writer.add_operation_str("push rbp");
    writer.add_operation_str("mov rbp, rsp");
    writer.add_operation_str("sub rsp, 16");
    writer.add_operation_str("mov rcx, 0");
    writer.add_operation_str("call exit");

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

    let object_file = String::from(build_path.join(format!("{}.obj", &NAME)).to_str().unwrap());
    let executable = String::from(build_path.join(format!("{}", &NAME)).to_str().unwrap());

    match execute(format!("gcc -o {}.exe {} -m64", executable, object_file)) {
        Ok(_out) => {}
        Err(error) => return Err(CompileError::GCC(error)),
    }

    return Ok(build_path.join(format!("{}.exe", NAME)));
}
