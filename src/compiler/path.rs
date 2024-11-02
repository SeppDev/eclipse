use std::path::PathBuf;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Path {
    pub components: Vec<String>,
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.components.join("/"))
    }
}

impl Path {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub fn from<T: ToString>(root: T) -> Self {
        let mut path = Self::new();
        path.add(root);
        return path;
    }
    pub fn join<T: ToString>(&self, seperator: T) -> String {
        self.components.join(&seperator.to_string())
    }
    pub fn add<T: ToString>(&mut self, name: T) {
        self.components.push(name.to_string())
    }
    // pub fn push(&mut self, path: &Self) {
    //     for path in &path.components {
    //         self.components.push(path.clone());
    //     }
    // }
    pub fn as_pathbuf(&self) -> PathBuf {
        let mut buf = PathBuf::new();
        for p in &self.components {
            buf.push(p);
        }
        return buf;
    }
    // pub fn from_pathbuf(path: &PathBuf) -> Self {
    //     let components = path.components();
    //     let mut path = Path::new();

    //     for component in components.into_iter() {
    //         path.add(String::from(component.as_os_str().to_str().unwrap()));
    //     }

    //     return path;
    // }
}
