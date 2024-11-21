use analyzer::analyze;
use counter::NameCounter;
use errors::{CompileMessages, CompileResult};
use parser::start_parse;
use path::Path;
use program::ParsedProgram;
use std::path::PathBuf;

mod analyzer;
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


pub fn build(project_dir: PathBuf) {
    let _executable = {
        let mut name_counter = NameCounter::new();
        let mut compile_messages = CompileMessages::new();

        // let standard = Path::new();
        // let mut standard = ParsedFile::new();
        // standard.imported.insert(String::from("io") );

        let main_path = Path::from("src").join("main");
        let main = match start_parse(&mut compile_messages, &project_dir, main_path) {
            Ok(file) => file,
            Err(()) => compile_messages.quit(),
        };
        compile_messages.throw(false);
        println!("{:#?}", main);

        let program = ParsedProgram {
            // standard,
            main,
        };

        let analyzed = match analyze(program, &mut compile_messages, &mut name_counter) {
            Ok(a) => a,
            Err(()) => compile_messages.quit()
        };
        compile_messages.throw(true);
        println!("{:#?}", analyzed);
    };
}

fn read_file(path: &PathBuf) -> CompileResult<String> {
    match std::fs::read_to_string(path) {
        Ok(source) => Ok(source),
        Err(error) => panic!("{:?}: {:?}", path, error),
    }
}
