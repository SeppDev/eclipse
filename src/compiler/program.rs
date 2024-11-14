use super::{
    errors::{CompileMessages, Location, Message, MessageKind},
    parser::ParsedFile,
    path::Path,
};

#[derive(Debug)]
pub struct ParsedProgram {
    pub standard: ParsedFile,
    pub main: ParsedFile,
}
impl ParsedProgram {
    pub fn get_file(&self, path: &Path /* , namespaces: &Vec<Path>*/) -> &ParsedFile {
        let mut components = path.components();
        components.reverse();
        let root = components.pop().unwrap();
        let mut file = match root.as_str() {
            "std" => &self.standard,
            _ => &self.main,
        };

        loop {
            let name = match components.pop() {
                Some(s) => s,
                None => break,
            };
            file = match file.imported.get(&name) {
                Some(file) => file,
                None => panic!("Could not find '{}'", name),
            }
        }
        return file;
    }
    // pub fn create_error<T: ToString, E: ToString>(
    //     &mut self,
    //     message: T,
    //     notice: E,
    //     relative_path: Path,
    //     location: Location,
    // ) -> &mut Message {
    //     self.errors
    //         .create(MessageKind::Error, relative_path, location, message, notice)
    // }
    // pub fn push_error<T: ToString, E: ToString>(
    //     &mut self,
    //     message: T,
    //     notice: E,
    //     relative_path: Path,
    //     location: Location,
    // ) {
    //     self.errors
    //         .create(MessageKind::Error, relative_path, location, message, notice);
    // }
}
