use super::parser::ParsedFile;

#[derive(Debug)]
pub struct ParsedProgram {
    pub standard: ParsedFile,
    pub main: ParsedFile
}