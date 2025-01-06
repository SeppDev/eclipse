use analyzer::analyze;
use codegen::codegen;
use errors::{CompileCtx, CompileResult};
use lib::get_std_file;
use optimizer::optimize;
use parser::{start_parse, ParsedFile};
use path::Path;
use std::{path::PathBuf, process::Output, time::Duration};

mod codegen;
mod lexer;
mod parser;
mod analyzer;
mod optimizer;

mod counter;
mod errors;
mod lib;
mod path; 
mod nodes;

pub static FILE_EXTENSION: &str = "ecl"; 

fn parse_program(ctx: &mut CompileCtx) -> CompileResult<Vec<ParsedFile>> {
    let mut files = Vec::new();
    
    let std_path = Path::from("std").join("mod");
    files.push(start_parse(ctx, std_path)?);

    let main_path = Path::from("src").join("main");
    files.push(start_parse(ctx, main_path)?);

    return Ok(files)
}

fn compile(ctx: &mut CompileCtx) -> CompileResult<PathBuf> {
    let build_path = ctx.project_dir.join("build");
    let build_file_path = build_path.join("build.ll");

    let mut executable_path = build_path.clone();
    ctx.target.set_extension(&mut executable_path);

    let files = parse_program(ctx)?;
    ctx.throw(false);
    
    let mut module = analyze(ctx, files);
    ctx.throw(false);
    
    ctx.set_status("Optimizing");
    optimize(ctx, &mut module);
    
    ctx.set_status("Codegen");
    let source = codegen(ctx, module);
    
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

pub fn build(project_dir: PathBuf) -> PathBuf {
    let mut ctx = CompileCtx::new(project_dir);
    let start = std::time::Instant::now();
    let path = compile(&mut ctx).unwrap_or_else(|_| ctx.quit());
    ctx.finish();

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
