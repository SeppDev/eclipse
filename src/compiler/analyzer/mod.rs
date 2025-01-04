use super::{errors::CompileCtx, parser::ParsedFile};

mod types;

pub fn analyze(ctx: &mut CompileCtx, mut files: Vec<ParsedFile>) {
    types::parse_types(ctx, &mut files);
}
