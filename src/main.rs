use std::{path::PathBuf, process::exit};

mod run;
use run::run;

mod compiler;

enum Action {
    Run,
    Build,
}

fn main() {
    let mut project_dir = std::env::current_dir().unwrap();
    let mut args = std::env::args();
    args.next();;

    let action = match args.next() {
        Some(a) => match a.as_str() {
            "run" => Action::Run,
            "build" => Action::Build,
            _ => {
                println!("Could not find action named: {}", a);
                exit(1)
            }
        },
        None => {
            println!("Argument required");
            exit(1)
        }
    };

    match args.next() {
        Some(path) => project_dir = PathBuf::from(path),
        None => {}
    };

    let executable_path = compiler::build(project_dir);
    match action {
        Action::Run => run(executable_path),
        Action::Build => {}
    }
}
