#[cfg(unix)]
pub fn execute(command: impl ToString) -> std::process::Output {
    use std::process::Command;

    Command::new("sh")
        .arg("-c")
        .arg(&command.to_string())
        .output()
        .expect("failed to execute process")
}

#[cfg(windows)]
pub fn execute(command: impl ToString) -> std::process::Output {
    use std::process::Command;

    Command::new("cmd")
        .args(["/C", &command.to_string()])
        .output()
        .expect("failed to execute process")
}
