use crate::compiler::{errors::CompileCtx, nodes::ast::RawNode, parser::ParsedFile};

pub struct ParsedTypes {}
impl ParsedTypes {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn parse_types(ctx: &mut CompileCtx, files: &mut Vec<ParsedFile>) {
    let mut types = ParsedTypes::new();
    for file in files {
        parse(ctx, &mut types, file);
        println!("{:#?}", file.body);
    }
}

fn parse(ctx: &mut CompileCtx, types: &mut ParsedTypes, file: &mut ParsedFile) {
    let (types, to_keep): (Vec<_>, Vec<_>) = file
        .body
        .drain(..)
        .partition(|node| matches!(node.raw, RawNode::Struct { .. }));

    for node in types {
        println!("{:#?}", node);
    }

    file.body = to_keep;
}
