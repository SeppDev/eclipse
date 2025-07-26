use std::path::PathBuf;

use context::CompilerCtx;

pub fn to_binary(compiler: &CompilerCtx, source: String) {
    let target = compiler.resolve_path(&PathBuf::from("target"));
    let build_file_path = target.join("build.ll");
    let final_path = target.join("build");

    let _ = std::fs::create_dir_all(&target);

    // let _ = std::fs::remove_file(&build_file_path);
    let _ = std::fs::remove_file(&final_path);

    let _ = std::fs::write(&build_file_path, source);

    let build_command = format!(
        "clang -O1 {} -o {}",
        build_file_path.to_string_lossy(),
        final_path.to_string_lossy()
    );

    let output = execute(build_command);

    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }
}

#[cfg(unix)]
pub fn execute(command: String) -> std::process::Output {
    use std::process::Command;

    Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("failed to execute process")
}

#[cfg(windows)]
pub fn execute(command: String) -> std::process::Output {
    use std::process::Command;

    Command::new("cmd")
        .args(["/C", &command])
        .output()
        .expect("failed to execute process")
}
