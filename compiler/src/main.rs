use cli::{CLI, command::Command};

fn main() {
    CLI::new("compiler").command(Command::new("build")).start();
}
