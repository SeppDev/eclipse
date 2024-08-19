use std::{io::{BufRead, BufReader}, panic, process::{Command, Stdio}};
use builder::compile;
use eclipse::CompileError;

mod parser;
mod builder;
mod analyzer;

pub const FILE_EXTENSION: &str = "eclipse";

fn main() {
    let executable = match build(String::new()) {
        Ok(path) => path,
        Err(error) => return handle_error(error)
    };
    run(executable);
}

fn build(project_path: String) -> Result<String, CompileError> {
    return compile(project_path);
}

fn run(executable_path: String) {
    let mut thread = Command::new(executable_path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = thread.stdout.as_mut().expect("Failed to open stdout");
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        match line {
            Ok(a) => println!("{}", a),
            Err(a) => println!("{:?}", a),
        }
    }

    thread.wait().unwrap();
}

fn handle_error(error: CompileError) {
    panic!("{:?}", error);


    // match error {
    //     CompileError::OpenFile(error) => panic!("{:?}", error),
    //     CompileError::Parsing(error) => panic!("{:#?}", error),
    //     CompileError::Building => panic!("Building error"),
    //     CompileError::GCC(response) => panic!("{}", response),
    //     CompileError::NASM(response) => panic!("{}", response),
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_test() {
        const SOURCE: &str = "C:/Users/Gebruiker/Documents/eclipse/first_project";
        // const NAME: &str = "app";

        let executable_path = match build(SOURCE.to_string()) {
            Ok(path) => path,
            Err(a) => {
                handle_error(a);
                panic!("Build failed")
            }
        };
        run(executable_path);
    }
}
