use analyzer::{analyze, ParsedProject};
use codegen::codegen;
pub use errors::{CompileCtx, CompileResult};
use lib::get_std_file;
use optimizer::optimize;
use parser::start_parse;
use path::Path;
use std::{path::PathBuf, process::Output, time::Duration};

mod analyzer;
mod codegen;
mod lexer;
mod optimizer;
mod parser;

mod counter;
mod errors;
mod lib;
mod nodes;
mod path;

pub static FILE_EXTENSION: &str = "ecl";

fn parse_program(ctx: &mut CompileCtx) -> CompileResult<ParsedProject> {
    let std_path = Path::from("std").join("mod");
    let std = start_parse(ctx, std_path)?;

    let main_path = Path::from("src").join("main");
    let main = start_parse(ctx, main_path)?;

    return Ok(ParsedProject { main, std });
}

fn compile(ctx: &mut CompileCtx) -> CompileResult<PathBuf> {
    let build_path = ctx.project_dir.join("build");
    let build_file_path = build_path.join("build.ll");

    let mut executable_path = build_path.join("build");
    ctx.target.set_extension(&mut executable_path);

    let project = parse_program(ctx)?;
    ctx.throw(false);

    let analyzed_module = analyze(ctx, project);
    ctx.throw(false);

    let analyzed_module = if ctx.options.release {
        ctx.set_status("Optimizing");
        optimize(ctx, analyzed_module)
    } else {
        analyzed_module
    };

    ctx.set_status("Codegen");
    let source = codegen(ctx, analyzed_module);

    let build_command = format!(
        "clang -O3 {} -o {}",
        build_file_path.to_string_lossy(),
        executable_path.to_string_lossy()
    );

    std::fs::create_dir_all(&build_path).unwrap();
    std::fs::write(&build_file_path, source).unwrap();

    let output = command(build_command);
    if !output.status.success() {
        ctx.result_print(format!("{}", String::from_utf8(output.stderr).unwrap()));
        ctx.quit();
    }

    return Ok(executable_path);
}

pub fn build(mut ctx: CompileCtx) -> PathBuf {
    let start = std::time::Instant::now();
    let path = compile(&mut ctx).unwrap_or_else(|_| ctx.quit());
    ctx.throw(true);

    let elapsed = start.elapsed();
    if elapsed > Duration::from_secs(1) {
        println!("Compiling took: {} seconds", elapsed.as_secs());
    }

    return path;
}

pub fn command(command: String) -> Output {
    use std::process::Command;

    return if cfg!(target_os = "windows") {
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
}

fn read_file(project_dir: &PathBuf, relative_file_path: &Path) -> String {
    if relative_file_path.first().unwrap() == &"std".to_string() {
        return get_std_file(relative_file_path).unwrap();
    }

    let mut full_path = project_dir.join(relative_file_path.into_path_buf());
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
    let mut full_path = project_dir.join(relative_file_path.into_path_buf());
    full_path.set_extension(FILE_EXTENSION);

    std::fs::exists(project_dir.join(full_path)).expect("Failed to read path")
}
