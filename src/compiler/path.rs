#[derive(Debug, PartialEq, Hash, Clone)]
pub struct Path {
    components: Vec<String>,
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.components.join("::"))
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
    pub fn add<T: ToString>(&mut self, name: T) {
        // self.components.push(Chain {
        //     method,
        //     body: name.to_string(),
        // })
        self.components.push(name.to_string());
    }
}

// #[derive(Debug, PartialEq, Hash, Clone)]
// struct Chain {
//     method: bool,
//     body: String,
// }
// impl Chain {
//     pub fn join(&self) -> String {
            
//     }
// }

// impl std::fmt::Display for Chain {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", "")
//         // if self.method {
//         // }
//     }
// }