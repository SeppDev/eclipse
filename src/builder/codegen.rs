use std::path::PathBuf;

use crate::{
    analyzer::analyzer::analyze,
    builder::writer::Writer,
    parser::{parse, Node},
    FILE_EXTENSION,
};
use eclipse::CompileError;
use scope::{scope, Function};

mod scope;

pub fn compile(project_path: PathBuf) -> Result<String, CompileError> {
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
                writer.label(&name);
                
                let mut function = Function::new();
                let mut scope_writer = Writer::new();
                

                scope(body, &mut writer, &mut function);
                writer.writer(scope_writer);
            }
            _ => panic!(),
        }
    }

    let build_path = project_path.join("build");
    std::fs::create_dir(&build_path).unwrap_or_default();
    match std::fs::write(build_path.clone().join("app.s"), writer.body) {
        Ok(()) => {},
        Err(error) => return Err(CompileError::OpenFile(error)),
    };
    // let file = std::fs::write(path, contents)


    // let file = File

    todo!()
}
