use std::path::PathBuf;


pub fn run(executable_path: PathBuf) {
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

    let mut thread = std::process::Command::new(executable_path)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let _ = tx.send(());

    let stdout = thread.stdout.as_mut().expect("Failed to open stdout");
    let reader = std::io::BufReader::new(stdout);

    for line in reader.lines() {
        match line {
            Ok(a) => println!("{}", a),
            Err(a) => println!("{:?}", a),
        }
    }

    let output = thread.wait().unwrap();
    if !output.success() {
        match output.code() {
            Some(code) => {
                println!("Program failed with code: {code}");
            }
            None => {
                println!("Program failed: {}", output.to_string())
            }
        }
    }
}