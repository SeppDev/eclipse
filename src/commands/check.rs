use crate::{
    cli::{arguments::Arguments, options::CommandLineOptions},
    common::exit::exit,
    compiler::CompilerCtx,
};

pub fn check(arguments: Arguments) {
    let options = CommandLineOptions::from(arguments);
    let mut compiler = CompilerCtx::new(options);
    compiler.analyze().unwrap_or_else(|f| exit(format!("{f}")));
}
