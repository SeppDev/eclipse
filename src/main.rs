use std::{env, io::{BufRead, BufReader}, panic, path::PathBuf, process::{Command, Stdio}};
use builder::compile;
use eclipse::CompileError;

mod parser;
mod builder;

pub const FILE_EXTENSION: &str = "eclipse";


#[derive(PartialEq, Eq)]
enum Action {
    Build,
    BuildAndRun,
}

fn main() {

    let project_dir = env::current_dir().unwrap();
    let mut arguments = env::args().into_iter().peekable();
    arguments.next().unwrap();

    let action = match arguments.next() {
        Some(action) => action,
        None => return println!("No argument was found.")
    };
    let action = match action.as_str() {
        "build" => Action::Build,
        "run" => Action::BuildAndRun,
        _ => return println!("{:?} is not a valid argument", action)
    };

    if action == Action::Build || action == Action::BuildAndRun {
        let executable = match build(project_dir) {
            Ok(path) => path,
            Err(error) => return handle_error(error)
        };

        if action == Action::BuildAndRun {
            run(executable);
        }
    }
}

fn build(project_path: PathBuf) -> Result<String, CompileError> {
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

        let executable_path = match build(PathBuf::from(SOURCE)) {
            Ok(path) => path,
            Err(a) => {
                handle_error(a);
                panic!("Build failed")
            }
        };
        run(executable_path);
    }
}
