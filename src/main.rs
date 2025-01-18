use std::{path::PathBuf, process::exit};

mod run;
use compiler::CompileCtx;
use run::run;

mod compiler;

enum Action {
    Run,
    Build,
}

fn main() {
    let mut project_dir = std::env::current_dir().unwrap();
    let mut args = std::env::args();
    args.next();

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

    let mut ctx = CompileCtx::new(project_dir);

    for arg in args {
        match arg.as_str() {
            "--release" => ctx.options.release = true,
            other => {
                let (key, value) = match other.split_once("=") {
                    Some(a) => a,
                    None => todo!(),
                };
                match key.to_lowercase().as_str() {
                    "project_path" => ctx.project_dir = PathBuf::from(value),
                    _ => todo!(),
                }
            }
        }
    }

    let executable_path = compiler::build(ctx);
    match action {
        Action::Run => run(executable_path),
        Action::Build => {}
    }
}
