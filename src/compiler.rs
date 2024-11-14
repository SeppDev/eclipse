use std::path::PathBuf;

use analyzer::analyze;
use counter::NameCounter;
use errors::CompileMessages;
use parser::{parse, ParsedFile};

mod analyzer;
mod lexer;
mod parser;

use path::Path;
use program::ParsedProgram;

mod counter;
mod errors;
mod path;
mod program;
mod string;
mod types;

pub static FILE_EXTENSION: &str = "ecl";
pub static POINTER_WIDTH: usize = 8;

fn parse_include(
    counter: &mut NameCounter,
    errors: &mut CompileMessages,
    source: &str,
    name: &str,
) -> (String, ParsedFile) {
    let mut relative_path = Path::from("src");
    relative_path.push(name);

    let mut file = parse(
        counter,
        errors,
        &PathBuf::new(),
        relative_path,
        source.to_string(),
    );
    file.export = true;
    return (name.to_string(), file);
}

pub fn build(project_dir: PathBuf) {
    let _executable = {
        let mut counter = NameCounter::new();
        let mut errors = CompileMessages::new();

        let std_imports = vec![
            parse_include(
                &mut counter,
                &mut errors,
                include_str!("./std/io.ecl"),
                "io",
            ),
            parse_include(
                &mut counter,
                &mut errors,
                include_str!("./std/math.ecl"),
                "math",
            ),
        ];

        let mut standard = ParsedFile::new();
        for (key, file) in std_imports {
            standard.imported.insert(key, file);
        }

        let relative_path = Path::from("src").join("main");
        let mut file_path = project_dir.join(relative_path.convert());
        file_path.set_extension(FILE_EXTENSION);

        let source = read_file(&file_path);
        let mut main = parse(
            &mut counter,
            &mut errors,
            &project_dir,
            relative_path,
            source,
        );
        main.export = true;

        let mut program = ParsedProgram {
            standard,
            main,
            errors,
        };

        let analyzed = analyze(&mut program);
        println!("{:#?}", analyzed);
    };
}

fn read_file(path: &PathBuf) -> String {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => panic!("{:?}: {:?}", path, error),
    };

    source
}
