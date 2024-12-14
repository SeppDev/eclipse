use analyzer::{analyze, parse_types, ProgramCtx};
use codegen::CodeGen;
use counter::NameCounter;
use errors::{CompileCtx, CompileResult};
use lib::get_std_file;
use parser::start_parse;
use path::Path;
use program::ParsedProgram;
use std::{path::PathBuf, process::Output, time::Duration};

mod analyzer;
mod codegen;
mod lexer;
mod parser;

mod counter;
mod errors;
mod lib;
mod path;
mod program;
mod string;
mod types;

pub static FILE_EXTENSION: &str = "ecl";
pub static POINTER_WITH: usize = 8;

fn parse_program(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    project_dir: &PathBuf,
) -> CompileResult<ParsedProgram> {
    let std_path = Path::from("std").join("mod");
    let standard = start_parse(debug, count, &PathBuf::new(), std_path.clone(), std_path)?;

    let main_path = Path::from("src").join("main");
    let main = start_parse(debug, count, project_dir, main_path.clone(), main_path)?;

    return Ok(ParsedProgram { standard, main });
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

    let mut program = parse_program(debug, count, &project_dir)?;
    debug.throw(false);

    let types = parse_types(debug, count, &mut program)?;
    debug.throw(false);

    // debug.result_print(format!("{:#?}", types));

    let mut ctx = ProgramCtx {
        debug,
        codegen: CodeGen::new(),
        types: &types,
        namespaces: &mut Vec::new()
        // count,
        // static_strings: &mut Vec::new(),
    };

    analyze(&mut ctx, program)?;
    ctx.debug.throw(true);

    ctx.debug.set_status("Building");

    let source = ctx.codegen.generate();

    let build_command = format!(
        "clang -O3 {} -o {}",
        build_file_path.to_string_lossy(),
        final_path.to_string_lossy()
    );

    std::fs::create_dir_all(&build_path).unwrap();
    std::fs::write(&build_file_path, source).unwrap();

    let output = execute(build_command);
    if !output.status.success() {
        debug.result_print(format!("{}", String::from_utf8(output.stderr).unwrap()));
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

    let elapsed = start.elapsed();
    if elapsed > Duration::from_secs(1) {
        println!("Compiling took: {} seconds", elapsed.as_secs());
    }

    return path;
}

pub fn execute(command: String) -> Output {
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &command])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("failed to execute process")
    } else {
        panic!("Operating system is not supported")
    };

    return output;
}

fn read_file(project_dir: &PathBuf, relative_file_path: &Path) -> String {
    if relative_file_path.first().unwrap() == &"std".to_string() {
        return get_std_file(relative_file_path).unwrap();
    }

    let mut full_path = project_dir.join(relative_file_path.convert());
    full_path.set_extension(FILE_EXTENSION);

    match std::fs::read_to_string(full_path) {
        Ok(source) => source,
        Err(error) => panic!("{:?}: {:?}", relative_file_path, error),
    }
}

fn file_exists(project_dir: &PathBuf, relative_file_path: &Path) -> bool {
    if relative_file_path.first().unwrap() == &"std".to_string() {
        return get_std_file(relative_file_path).is_some();
    }
    let mut full_path = project_dir.join(relative_file_path.convert());
    full_path.set_extension(FILE_EXTENSION);

    std::fs::exists(project_dir.join(full_path)).expect("Failed to read path")
}
