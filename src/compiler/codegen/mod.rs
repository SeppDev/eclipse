use transform::transform;

use super::{analyzer::AnalyzedModule, errors::CompileCtx};

pub mod variables;

pub mod target;
mod transform;

pub struct Source {
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
    fn tpushln<T: ToString>(&mut self, contents: T) {
        self.push("\t");
        self.push(contents);
        self.body.push('\n');
    }
}

pub fn codegen(ctx: &mut CompileCtx, module: AnalyzedModule) -> String {
    let mut source = Source::new();
    let ir = transform(ctx, module);

    source.pushln(format!("target triple = \"{}\"", ctx.target));

    for function in ir.functions {
        source.pushln(format!(
            "define {} @{}({}) {{\nstart:\n {}}}\n",
            function.return_type,
            function.key,
            function
                .parameters
                .into_iter()
                .map(|(dt, key)| format!("{dt} %{key}"))
                .collect::<Vec<String>>()
                .join(", "),
            function.body.body
        ))
    }

    return source.body;
}
