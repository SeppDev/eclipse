use std::{env, path::PathBuf};

use clap::{Parser, Subcommand};
use common::cmd::execute;
use compiler::CompilerCtx;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Check,
    // Build,
    Run {
        #[arg(long, value_name = "PATH")]
        project: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    let command = match cli.command {
        Some(c) => c,
        None => return,
    };

    match command {
        Commands::Run { project } => {
            let mut compiler = CompilerCtx::builder()
                .project_path(project.unwrap_or_else(|| env::current_dir().unwrap()))
                .build();

            let executable = compiler::compile(&mut compiler);

            execute(format!("{executable:#?}"))
        }
        _ => todo!(),
    };
}
