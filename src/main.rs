use cli::CLI;

mod cli;
mod commands;
mod common;
mod compiler;
mod diagnostics;
mod utils;

pub const FILE_EXTENSION: &str = "ecl";

fn main() {
    CLI::new()
        .register(
            "Analyzes current project",
            vec!["c", "check"],
            commands::check,
        )
        .register(
            "Builds current project",
            vec!["b", "build"],
            commands::build,
        )
        .register(
            "Build and runs current project",
            vec!["r", "run"],
            commands::run,
        )
        .register(
            "Create a new project <path>",
            vec!["n", "new"],
            commands::new,
        )
        .start();
}
