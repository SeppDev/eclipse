use crate::cli::{arguments::Arguments, options::CommandLineOptions};

pub fn build(arguments: Arguments) {
    let options = CommandLineOptions::from(arguments);
    println!("Building!")
}
