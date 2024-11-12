use super::{parser::ParsedFile, path::Path};

#[derive(Debug)]
pub struct ParsedProgram {
    pub standard: ParsedFile,
    pub main: ParsedFile,
}
impl ParsedProgram {
    pub fn get_file(&self, path: &Path/* , namespaces: &Vec<Path>*/) -> &ParsedFile {
        let mut components = path.components();
        components.reverse();
        let root = components.pop().unwrap();
        let mut file = match root.as_str() {
            "std" => &self.standard,
            _ => &self.main  
        };

        loop {
            let name = match components.pop() {
                Some(s) => s,
                None => break
            };
            file = match file.imported.get(&name) {
                Some(file) => file,
                None => panic!("Could not find '{}'", name)
            }
        }
        return file;
    }
}