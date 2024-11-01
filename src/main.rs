use eclipse::{build, CompileArguments, FILE_EXTENSION};
use std::{
    env,
    io::BufReader,
    path::PathBuf,
    process::{exit, Command, Stdio},
};


fn main() {
    #[derive(PartialEq, Eq)]
    enum Action {
        New,
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
        "new" => Action::New,
        _ => return println!("{:?} is not a valid argument", action),
    };

    let mut c_arguments = CompileArguments::new();
    loop {
        let arg = match arguments.next() {
            Some(arg) => arg,
            None => break
        };
        c_arguments.insert(arg);
    }

    if action == Action::Build || action == Action::BuildAndRun {
        let executable = match build(project_dir, c_arguments) {
            Ok(path) => path,
            Err(a) => {
                a.print();
                exit(1)
            }
        };

        if action == Action::BuildAndRun {
            run(executable);
        }
    } else if action == Action::New {
        let name = match arguments.next() {
            Some(name) => name,
            None => return println!("No name specified"),
        };

        let mut path: PathBuf = match arguments.next() {
            Some(path) => PathBuf::from(path),
            None => project_dir.to_path_buf(),
        };

        // Check if the project already exists
        path = path.join(format!("{}", name));
        if path.exists() == true {
            return println!("{:?} already exists", path);
        }

        match std::fs::create_dir(&path) {
            Ok(_) => {}
            Err(error) => return println!("{:?}", error),
        };

        // Create the src directory
        path = path.join("src");
        match std::fs::create_dir(&path) {
            Ok(_) => {}
            Err(error) => return println!("{:?}", error),
        };

        // Create the main file
        match std::fs::write(
            path.join(format!("main.{}", FILE_EXTENSION)),
            "fn main() {\n\tprint(42);\n}",
        ) {
            Ok(_) => {}
            Err(error) => return println!("{:?}", error),
        };
    }
}

fn run(executable_path: PathBuf) {
    use std::io::BufRead;
    use std::sync::mpsc::TryRecvError;
    use std::thread;

    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {
                println!("Spawning thread is taking longer than 2 seconds");
                break;
            }
        }
    });

    let mut thread = Command::new(executable_path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _ = tx.send(());

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
        let mut path = PathBuf::from("C:/Users/Gebruiker/Documents/eclipse/first_project/");
        if !path.exists() {
            path = PathBuf::from("C:/Users/seppd/OneDrive/Documenten/Eclipse/first_project/");
        }

        let executable_path = match build(path, CompileArguments::test()) {
            Ok(path) => path,
            Err(a) => {
                a.print();
                exit(1)
            }
        };
        run(executable_path);
    }
}
