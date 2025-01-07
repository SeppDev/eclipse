use transform::transform;

use super::{analyzer::AnalyzedModule, errors::CompileCtx};

pub mod target;
mod transform;

struct Source {
    pub body: String,
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

pub fn codegen(ctx: &mut CompileCtx, module: AnalyzedModule) -> String {
    let mut source = Source::new();
    let ir = transform(ctx, module);

    source.pushln(format!("target triple = \"{}\"", ctx.target));

    for function in ir.functions {
        let mut body = Source::new();
        
        for instruction in function.body {            
            body.pushln(format!("{instruction}"));
        };
        
        source.pushln(format!(
            "define {} @{}() {{\nstart:\n {} \n}}",
            function.return_type,
            function.key,
            body.body
        ))
    }

    return source.body;
}
