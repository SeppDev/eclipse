use analyzer::analyze;
use borrowcheck::borrow_check;
use build::to_binary;
use context::CompilerCtx;
use lowering::lower_to_mir;
use resolver::resolve_modules;

mod build;

pub fn compile(compiler: &mut CompilerCtx) {
    let entry = CompilerCtx::entry();

    let collection = resolve_modules(compiler, &entry);
    let collection = analyze(compiler, collection);
    let collection = borrow_check(compiler, collection);

    let module = lower_to_mir(compiler, collection);
    let source = codegen::generate(compiler, module);

    println!("{source:#?}");

    to_binary(compiler, source);
}
