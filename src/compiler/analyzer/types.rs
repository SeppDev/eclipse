use crate::compiler::{errors::CompileCtx, nodes::ast::RawLayout, parser::ParsedFile};

pub fn parse_types(ctx: &mut CompileCtx, files: &mut Vec<ParsedFile>) {
    for file in files {
        handle_file(ctx, file)
    }
}

fn handle_file(ctx: &mut CompileCtx, file: &mut ParsedFile) {
    for (_, file) in &mut file.imports {
        handle_file(ctx, file);
    }
    
    for layout in file.layouts.drain(..).into_iter() {
        match layout.raw {
            RawLayout::Struct { name, fields } => continue,
            RawLayout::Enum { name, fields } => continue,
        }
    }
}