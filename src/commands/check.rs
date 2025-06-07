use crate::{
    cli::{arguments::Arguments, options::CommandLineOptions},
    compiler::CompilerCtx,
};

pub fn check(arguments: Arguments) {
    let mut compiler: CompilerCtx = CommandLineOptions::from(arguments).into();
    let modules = compiler.parse();
    let _modules = compiler.analyze(&modules);

    compiler.finish();
}
