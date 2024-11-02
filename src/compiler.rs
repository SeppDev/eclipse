use std::path::PathBuf;

use lexer::tokenize;

mod lexer;
mod path;
mod string;

pub static FILE_EXTENSION: &str = "ecl";

pub fn build(project_dir: PathBuf) {
    let mut main_path = project_dir.join("src/main");
    main_path.set_extension(FILE_EXTENSION);
    let main = read_file(&main_path);
    let tokens = tokenize(main);
    println!("{:#?}", tokens);
}

fn read_file(path: &PathBuf) -> String {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => todo!("{:?}", error)
    };

    source
}