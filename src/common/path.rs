use std::{fmt::Debug, path::PathBuf};

#[derive(PartialEq, Eq, Hash, Clone, Default)]
pub struct Path {
    components: Vec<String>,
    extension: Option<String>,
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let body = self.components.join("/");
        let mut extension: String = String::new();

        if let Some(ext) = &self.extension {
            extension.push('.');
            extension.push_str(ext.as_str());
        };

        write!(f, "{body}{extension}")
    }
}
impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Into<PathBuf> for Path {
    fn into(self) -> PathBuf {
        self.as_path_buf()
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
    pub fn stringify(&self, sep: &str) -> String {
        self.components.join(sep)
    }
    pub fn single(root: &str) -> Self {
        let mut path = Self::new();
        path.push(root);
        return path;
    }
    pub fn extension(mut self, extension: &str) -> Self {
        self.set_extension(extension);
        self
    }
    pub fn set_extension(&mut self, extension: &str) {
        if extension.len() == 0 {
            self.extension = None
        } else {
            self.extension = Some(extension.into())
        }
    }
    pub fn exists(&self) -> bool {
        self.as_path_buf().exists()
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
    pub fn join(&self, name: &str) -> Self {
        self.clone().extend_single(name)
    }
    pub fn extend_single(mut self, name: &str) -> Self {
        self.components.push(name.to_string());
        self
    }
    pub fn extend(mut self, other: &Path) -> Path {
        for name in other.components.iter() {
            self.push(name);
        }
        if let Some(ext) = &other.extension {
            self.set_extension(ext);
        }
        self
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
    pub fn as_path_buf(&self) -> PathBuf {
        let mut path: PathBuf = self.components.iter().collect();
        if let Some(extension) = &self.extension {
            path.set_extension(extension);
        }
        path
    }
}
