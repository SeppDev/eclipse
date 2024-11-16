use analyzer::analyze;
use counter::NameCounter;
use errors::CompileMessages;
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
        let main = start_parse(
            &mut name_counter,
            &mut compile_messages,
            &project_dir,
            main_path,
        );
        compile_messages.throw(false);

        let mut program = ParsedProgram {
            // standard,
            main,
        };

        let _analyzed = analyze(&mut program, &mut compile_messages);
        compile_messages.throw(true);
        // println!("{:#?}", analyzed);
    };
}

fn read_file(path: &PathBuf) -> String {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => panic!("{:?}: {:?}", path, error),
    };

    source
}
