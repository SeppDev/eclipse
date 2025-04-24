use crate::{cli::{arguments::Arguments, options::CommandLineOptions}, compiler::CompilerCtx};

pub fn build(arguments: Arguments) {
    let compiler: CompilerCtx = CommandLineOptions::from(arguments).into();
    println!("Building!")
}
