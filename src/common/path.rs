use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Path {
    components: Vec<String>,
    extension: Option<String>,
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let body = self.components.join("/");
        let extension: &str = match &self.extension {
            Some(extension) => extension,
            _ => "",
        };

        write!(f, "{body:?}.{extension}")
    }
}
impl Into<PathBuf> for Path {
    fn into(self) -> PathBuf {
        let mut path: PathBuf = self.components.iter().collect();
        if let Some(extension) = self.extension {
            path.set_extension(extension);
        }
        path
    }
}
impl Into<Path> for PathBuf {
    fn into(self) -> Path {
        let extension: Option<String> = self
            .extension()
            .and_then(|e| e.to_str())
            .and_then(|o| Some(o.into()));

        let components: Vec<String> = self
            .into_iter()
            .map(|o| o.to_str().unwrap())
            .map(|o| o.to_string())
            .collect();

        Path {
            components,
            extension,
        }
    }
}

impl Path {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            extension: None,
        }
    }
    pub fn single(root: &str) -> Self {
        let mut path = Self::new();
        path.push(root);
        return path;
    }
    pub fn set_extension(&mut self, extension: &str) {
        if extension.len() == 0 {
            self.extension = None
        } else {
            self.extension = Some(extension.into())
        }
    }
    pub fn join(&self, name: &str) -> Self {
        let mut new = self.clone();
        new.push(name);
        return new;
    }
    pub fn push(&mut self, name: &str) {
        self.components.push(name.into());
    }
    pub fn pop(&mut self) -> Option<String> {
        self.components.pop()
    }
    pub fn parent(&self) -> Self {
        let mut clone = self.clone();
        clone.pop();
        clone
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
