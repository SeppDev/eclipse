use eclipse::build;
use std::{
    env,
    io::{BufRead, BufReader},
    panic,
    process::{Command, Stdio},
};


fn main() {
    #[derive(PartialEq, Eq)]
    enum Action {
        Build,
        BuildAndRun,
    } 

    let project_dir = env::current_dir().unwrap();
    let mut arguments = env::args().into_iter().peekable();
    arguments.next().unwrap();

    let action = match arguments.next() {
        Some(action) => action,
        None => return println!("No argument was found."),
    };
    let action = match action.as_str() {
        "build" => Action::Build,
        "run" => Action::BuildAndRun,
        _ => return println!("{:?} is not a valid argument", action),
    };

    if action == Action::Build || action == Action::BuildAndRun {
        let executable = match build(project_dir) {
            Ok(path) => path,
            Err(a) => {
                a.print();
                panic!()
            }
        };

        if action == Action::BuildAndRun {
            run(executable);
        }
    }
    // math::add_one(1);
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
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn build_test() {
        const SOURCE: &str = "C:/Users/Gebruiker/Documents/eclipse/first_project/";
        // const NAME: &str = "app";

        let executable_path = match build(PathBuf::from(SOURCE)) {
            Ok(path) => path,
            Err(a) => {
                a.print();
                panic!()
            }
        };
        run(executable_path);
    }
}
