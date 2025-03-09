use crate::cli::{arguments::Arguments, options::CommandLineOptions};

pub fn run(arguments: Arguments) {
    let _options = CommandLineOptions::from(arguments);
    println!("Running!")
}
