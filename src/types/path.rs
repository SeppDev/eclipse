
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
    pub fn from(root: String) -> Self {
        let mut path = Self::new();
        path.add(root);
        return path;
    }
    pub fn join(&self, seperator: String) -> String {
        self.components.join(&seperator)
    }
    pub fn add(&mut self, name: String) {
        self.components.push(name)
    }
    // pub fn push(&mut self, path: &Self) {
    //     for path in &path.components {
    //         self.components.push(path.clone());
    //     }
    // }
    // pub fn to_pathbuf(&self) -> PathBuf {
    //     let mut buf = PathBuf::new();
    //     for p in &self.components {
    //         buf.push(p);
    //     }
    //     return buf;
    // }
    // pub fn from_pathbuf(path: &PathBuf) -> Self {
    //     let components = path.components();
    //     let mut path = Path::new();

    //     for component in components.into_iter() {
    //         path.add(String::from(component.as_os_str().to_str().unwrap()));
    //     }

    //     return path;
    // }
}
