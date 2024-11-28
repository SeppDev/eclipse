use analyzer::{analyze, parse_types};
use codegen::codegen;
use counter::NameCounter;
use errors::{CompileCtx, CompileResult};
use parser::start_parse;
use path::Path;
use program::ParsedProgram;
use std::{path::PathBuf, process::Output};

mod analyzer;
mod codegen;
mod lexer;
mod parser;

mod counter;
mod errors;
mod path;
mod program;
mod string;
mod types;

pub static FILE_EXTENSION: &str = "ecl";
// pub static POINTER_WIDTH: usize = 8;

fn parse_program(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    project_dir: &PathBuf,
) -> CompileResult<ParsedProgram> {
    let main_path = Path::from("src").join("main");
    let main = start_parse(debug, count, project_dir, main_path.clone(), main_path)?;

    // let main_path = Path::from("src").join("main");
    // let standard = start_parse(debug, count, project_dir, relative_file_path);

    return Ok(ParsedProgram {
        // standard,
        main,
    });
}

fn compile(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    project_dir: &PathBuf,
) -> CompileResult<PathBuf> {
    let build_path = project_dir.join("build");
    let build_file_path = build_path.join("build.ll");
    let final_path = build_path.join("build.exe");

    let _ = std::fs::remove_file(&build_file_path);
    let _ = std::fs::remove_file(&final_path);


    let program = parse_program(debug, count, &project_dir)?;
    debug.throw(false);

    let types = parse_types(debug, count, &program)?;
    debug.throw(false);

    let analyzed = analyze(debug, count, types, program)?;
    debug.throw(true);

    debug.set_status("Building");

    let source = codegen(analyzed);


    let build_command = format!(
        "clang -O3 {} -o {}",
        build_file_path.to_string_lossy(),
        final_path.to_string_lossy()
    );

    std::fs::create_dir_all(&build_path).unwrap();
    std::fs::write(&build_file_path, source).unwrap();

    let output = execute(build_command);
    if !output.status.success() {
        println!("{}", String::from_utf8(output.stderr).unwrap());
        debug.quit();
    }

    return Ok(final_path);
}

pub fn build(project_dir: PathBuf) -> PathBuf {
    let mut debug = CompileCtx::new();
    let mut count = NameCounter::new();

    let start = std::time::Instant::now();

    let path = match compile(&mut debug, &mut count, &project_dir) {
        Ok(p) => p,
        Err(()) => debug.quit(),
    };
    
    debug.finish();

    println!("Compiling took: {} seconds", start.elapsed().as_secs());

    return path;
}

pub fn execute(command: String) -> Output {
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("failed to execute process")
    };

    return output;
}

fn read_file(path: &PathBuf) -> CompileResult<String> {
    match std::fs::read_to_string(path) {
        Ok(source) => Ok(source),
        Err(error) => panic!("{:?}: {:?}", path, error),
    }
}
