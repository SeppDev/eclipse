use crate::cli::{arguments::Arguments, options::CommandLineOptions};

pub fn run(arguments: Arguments) {
    let options = CommandLineOptions::from(arguments);
    println!("Running!")
}
