use std::path::PathBuf;

#[derive(Debug, Eq, Hash, Clone)]
pub struct Path {
    file_extension: Option<String>,
    components: Vec<String>,
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.components.join("/"))
    }
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl From<&PathBuf> for Path {
    fn from(value: &PathBuf) -> Self {
        let file_extension = match value.extension() {
            Some(f) => Some(f.to_str().unwrap().to_string()),
            None => None,
        };
        let components = value
            .components()
            .map(|c| c.as_os_str().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        return Path {
            components,
            file_extension,
        };
    }
}

impl Path {
    pub fn new() -> Self {
        Self {
            file_extension: None,
            components: Vec::new(),
        }
    }
    pub fn single<T: ToString>(root: T) -> Self {
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
    pub fn into_path_buf(&self) -> PathBuf {
        let mut buf = PathBuf::new();
        for p in &self.components {
            buf.push(p);
        }
        match &self.file_extension {
            Some(ext) => buf.set_extension(ext.clone()),
            None => false,
        };
        buf
    }
    pub fn len(&self) -> usize {
        return self.components.len();
    }
    pub fn first(&self) -> Option<&String> {
        return self.components.first();
    }
    pub fn last(&self) -> Option<&String> {
        return self.components.last();
    }
}
