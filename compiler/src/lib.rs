use std::path::PathBuf;

use analyzer::analyze;
use borrowcheck::borrow_check;
use build::to_binary;
pub use context::CompilerCtx;
use lowering::lower_to_mir;
use resolver::resolve_modules;

mod build;

pub fn compile(compiler: &mut CompilerCtx) -> PathBuf {
    let entry = CompilerCtx::entry();

    let collection = resolve_modules(compiler, &entry);
    let collection = analyze(compiler, collection);
    let collection = borrow_check(compiler, collection);

    let module = lower_to_mir(compiler, collection);
    println!("{module:#?}");

    let source = codegen::generate(compiler, module);

    println!("{source}");

    to_binary(compiler, source)
}
