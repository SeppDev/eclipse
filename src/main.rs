use cli::{arguments::Arguments, CLI};
// use common::errors::CompileResult;
// use compiler::context::CompileCtx;

mod cli;
mod common;
// mod compiler;
// mod lsp;
// mod utils;

fn check(arguments: Arguments) {
    println!("Checking!")
}

fn main() {
    CLI::new()
        .register("Analyzes current project", vec!["c", "check"], check)
        .start();
}

// fn _main() {
//     let error = match run() {
//         Ok(()) => return,
//         Err(err) => err,
//     };

//     eprintln!("{error}");
// }

// fn run() -> CompileResult<()> {
//     let mut arguments = common::arguments::Arguments::new();

//     let command = match arguments.next() {
//         Some(command) => match &command[..] {
//             "r" | "run" => Command::Run,
//             "b" | "build" => Command::Build,
//             "c" | "check" => Command::Check,
//             "new" => Command::New,
//             "lsp" => Command::LSP,
//             _ => common::exit(format!("Could not find command named: '{command}'")),
//         },
//         None => common::exit("Missing command argument"),
//     };

//     if let Command::New = command {
//         todo!()
//     }

//     let mut ctx = CompileCtx::new(arguments)?;

//     let start = std::time::Instant::now();

//     match command {
//         Command::LSP => todo!(),
//         Command::Run => todo!(),
//         Command::Build => todo!(),
//         Command::Check => ctx.analyze()?,
//         _ => unreachable!(),
//     }
//     let elapsed = start.elapsed();

//     ctx.finish();

//     println!("Elapsed: {:?}", elapsed);
//     Ok(())
// }
