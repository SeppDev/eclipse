use crate::cli::{arguments::Arguments, options::CommandLineOptions};

pub fn build(arguments: Arguments) {
    let _options = CommandLineOptions::from(arguments);
    println!("Building!")
}
