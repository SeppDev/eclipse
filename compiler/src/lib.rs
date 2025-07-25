use analyzer::analyze;
use context::CompilerCtx;
use resolver::resolve_modules;

pub fn compile(compiler: &mut CompilerCtx) {
    let entry = CompilerCtx::entry();

    let collection = resolve_modules(compiler, &entry);
    let hlir = analyze(compiler, collection);

    println!("{hlir:#?}");
}
