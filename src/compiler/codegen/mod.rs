use transform::transform;

use super::{analyzer::AnalyzedModule, errors::CompileCtx};

pub mod target;
mod transform;

struct Source {
    body: String,
}

impl Source {
    fn new() -> Self {
        Self {
            body: String::new(),
        }
    }
    fn push<T: ToString>(&mut self, contents: T) {
        self.body.push_str(contents.to_string().as_str());
    }
    fn pushln<T: ToString>(&mut self, contents: T) {
        self.push(contents);
        self.body.push('\n');
    }
}

pub fn codegen(ctx: &CompileCtx, module: AnalyzedModule) -> String {
    let mut source = Source::new();
    let ir = transform(ctx, module);

    source.pushln(format!("target triple = \"{}\"", ctx.target));

    source.pushln(
        ir.functions
            .into_iter()
            .map(|function| format!("{function}"))
            .collect::<String>(),
    );

    return source.body;
}
