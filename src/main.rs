use common::{errors::CompileResult, exit_code::ExitCode};
use compiler::context::CompileCtx;

mod commands;
mod common;
mod compiler;
mod lsp;

enum Command {
    LSP,
    Run,
    Build,
    Check,
    New,
}

fn main() {
    let error = match run() {
        Ok(()) => return,
        Err(err) => err,
    };

    eprintln!("{error}");
}

fn run() -> CompileResult<()> {
    let mut arguments = common::arguments::Arguments::new();

    let command = match arguments.next() {
        Some(command) => match &command[..] {
            "r" | "run" => Command::Run,
            "b" | "build" => Command::Build,
            "c" | "check" => Command::Check,
            "new" => Command::New,
            "lsp" => Command::LSP,
            _ => common::exit(
                format!("Could not find command named: '{command}'"),
                ExitCode::MissingCommand,
            ),
        },
        None => common::exit("Missing command argument", ExitCode::MissingCommand),
    };

    if let Command::New = command {
        todo!()
    }

    let mut ctx = CompileCtx::new(arguments)?;

    let start = std::time::Instant::now();

    match command {
        Command::LSP => todo!(),
        Command::Run => todo!(),
        Command::Build => todo!(),
        Command::Check => ctx.analyze()?,
        _ => unreachable!(),
    }
    let elapsed = start.elapsed();

    ctx.finish();

    println!("Elapsed: {:?}", elapsed);
    Ok(())
}
