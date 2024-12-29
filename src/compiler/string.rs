#[derive(Debug, Default)]
pub struct BetterString {
    body: String,
}
impl BetterString {
    pub fn new() -> Self {
        Self {
            body: String::new(),
        }
    }
    pub fn to_string(self) -> String {
        self.body
    }
    pub fn from<T: ToString>(value: T) -> Self {
        Self {
            body: value.to_string(),
        }
    }
    pub fn next_line(&mut self) {
        self.body.push('\n');
    }
    pub fn push<T: ToString>(&mut self, value: T) {
        self.body.push_str(value.to_string().as_str());
    }
    pub fn pushln<T: ToString>(&mut self, value: T) {
        self.push(value);
        self.body.push('\n');
    }
}
impl std::fmt::Display for BetterString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}
