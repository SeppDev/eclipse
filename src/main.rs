use eclipse::{self, BuildError};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
mod compiler;
mod lexer;
mod parser;

pub const FILE_EXTENSION: &str = "eclipse";

use std::env;

enum Action {
    Build,
    BuildAndRun,
    New,
}

fn help() {
    // print all commands
    todo!()
}

fn main() {
    let mut args = env::args().into_iter();
    let _run_path = args.next().unwrap();

    // TODO handle exceptions
    let action: Action = match args.next() {
        Some(a) => match a.to_lowercase().as_str() {
            "run" => Action::BuildAndRun,
            "build" => Action::Build,
            arg => panic!("No such command: {:?}", arg),
        },
        None => {
            help();
            todo!()
        }
    };
    let path = match args.next() {
        Some(path) => path,
        None => panic!("Expected path"),
    };

    match action {
        Action::Build => {
            build(path, String::from("app")).unwrap();
        }
        Action::BuildAndRun => {
            let executable = build(path, String::from("app")).unwrap();
            run(executable);
        },
        Action::New => todo!()
    }
}

fn build(project_path: String, name: String) -> Result<String, BuildError> {
    return compiler::compiler::build(project_path, name);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_test() {
        const SOURCE: &str = "C:/Users/Gebruiker/Documents/eclipse/first_project";
        const NAME: &str = "test";

        let executable_path = match build(SOURCE.to_string(), NAME.to_string()) {
            Ok(path) => path,
            Err(a) => panic!("{:?}", a),
        };
        run(executable_path);
    }
}
