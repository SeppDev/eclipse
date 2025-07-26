pub trait Appendable {
    fn line(&mut self);
    fn push_string(&mut self, value: std::string::String);
    fn pushln<T: ToString>(&mut self, value: T);
}

impl Appendable for String {
    fn line(&mut self) {
        self.push('\n');
    }

    fn push_string(&mut self, value: String) {
        self.push_str(&value);
    }

    fn pushln<T: ToString>(&mut self, value: T) {
        self.push_string(value.to_string());
        self.line();
    }
}
