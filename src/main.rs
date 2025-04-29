mod cli;
mod commands;
mod common;
mod compiler;
mod utils;

pub const FILE_EXTENSION: &str = "ecl";

fn main() {
    cli::CLI::new()
        .register(
            "analyzes current project",
            vec!["c", "check"],
            commands::check,
        )
        .register(
            "builds current project",
            vec!["b", "build"],
            commands::build,
        )
        .register(
            "build and runs current project",
            vec!["r", "run"],
            commands::run,
        )
        .register(
            "create a new project <path>",
            vec!["n", "new"],
            commands::new,
        )
        .register("initialize project", vec!["init"], commands::init)
        .start()
}
