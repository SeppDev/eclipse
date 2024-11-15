use std::path::PathBuf;

#[derive(Debug, PartialEq, Hash, Clone, Default)]
pub struct Path {
    components: Vec<String>,
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
    pub fn convert(&self) -> PathBuf {
        let mut p = PathBuf::new();
        for a in &self.components {
            p.push(a);
        }
        return p;
    }
    pub fn from<T: ToString>(root: T) -> Self {
        let mut path = Self::new();
        path.push(root);
        return path;
    }
    pub fn join<T: ToString>(&self, name: T) -> Self {
        let mut new = self.clone();
        new.push(name);
        return new;
    }
    pub fn push<T: ToString>(&mut self, name: T) {
        self.components.push(name.to_string());
    }
    pub fn pop(&mut self) -> Option<String> {
        self.components.pop()
    }
    pub fn parent(&self) -> Self {
        let mut clone = self.clone();
        clone.pop();
        clone
    }
    pub fn first(&self) -> Option<&String> {
        return self.components.first();
    }
    pub fn len(&self) -> usize {
        self.components.len()
    }
    pub fn components(&self) -> Vec<String> {
        return self.components.clone()
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