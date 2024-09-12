use std::path::PathBuf;

use crate::analyzer::analyze;
use crate::codegen::generate;
use crate::assembler::assemble;
use crate::parser::Program;
use crate::CompileError;

use crate::FILE_EXTENSION;

pub fn build(project_path: PathBuf) -> Result<PathBuf, CompileError> {
    let mut program = Program::new(project_path.clone());
    let program = match program.parse(PathBuf::from(format!("src/main.{}", FILE_EXTENSION))) {
        Ok(()) => match analyze(program) {
            Ok(p) => p,
            Err(error) => return Err(error)
        }
        Err(error) => return Err(error),
    };

    let ir_program = match generate(program) {
        Ok(p) => p,
        Err(error) => return Err(error)
    };
    
    let build_path = project_path.join("build");
    match std::fs::create_dir_all(&build_path) {
        Ok(()) => {},
        Err(error) => panic!("{:?}", error)
    };

    let executable = match assemble(ir_program, build_path) {
        Ok(path) => path,
        Err(error) => return Err(error)
    };

    return Ok(executable);
}
