use analyzer::analyze;
use codegen::codegen;
use counter::NameCounter;
use errors::{CompileMessages, CompileResult, DebugInfo};
use parser::start_parse;
use path::Path;
use program::ParsedProgram;
use std::{path::PathBuf, process::{exit, Output}};

mod analyzer;
mod codegen;
mod lexer;
mod parser;

mod counter;
mod errors;
mod path;
mod program;
mod string;
mod types;

pub static FILE_EXTENSION: &str = "ecl";
// pub static POINTER_WIDTH: usize = 8;

fn handle_debug_info(mut compile_messages: CompileMessages, info: DebugInfo) -> ! {
    compile_messages.push(info.relative_file_path, info.message);
    compile_messages.quit();
}

pub fn build(project_dir: PathBuf) -> PathBuf  {
    let executable = {
        let mut name_counter = NameCounter::new();
        let mut compile_messages = CompileMessages::new();

        let main_path = Path::from("src").join("main");
        let main = match start_parse(
            &mut compile_messages,
            &mut name_counter,
            &project_dir,
            main_path,
        ) {
            Ok(file) => file,
            Err(info) => handle_debug_info(compile_messages, info),
        };
        compile_messages.throw(false);

        let program = ParsedProgram {
            // standard,
            main,
        };

        let analyzed = match analyze(program, &mut compile_messages, &mut name_counter) {
            Ok(a) => a,
            Err(info) => handle_debug_info(compile_messages, info),
        };
        compile_messages.throw(true);
        let source = codegen(analyzed);

        let build_path = project_dir.join("build");
        let build_file_path = build_path.join("build.ll");
        let final_path = build_path.join("build.exe");

        let build_command = format!(
            "clang -O3 {} -o {}",
            build_file_path.to_string_lossy(),
            final_path.to_string_lossy()
        );

        std::fs::create_dir_all(&build_path).unwrap();

        std::fs::write(&build_file_path, source).unwrap();

        let output = execute(build_command).unwrap();
        if !output.status.success() {
            println!("{}", String::from_utf8(output.stderr).unwrap());
            exit(2)
        }

        final_path
    };

    return executable
}

pub fn execute(command: String) -> Result<Output, String> {
    let cmd = match std::process::Command::new("cmd")
        .args(["/C", &command])
        .output()
    {
        Ok(a) => a,
        Err(a) => return Err(a.to_string()),
    };

    return Ok(cmd);
}

fn read_file(path: &PathBuf) -> CompileResult<String> {
    match std::fs::read_to_string(path) {
        Ok(source) => Ok(source),
        Err(error) => panic!("{:?}: {:?}", path, error),
    }
}
