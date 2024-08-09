use std::collections::HashMap;

// const CHARS: [&str; 16] = ["a", "b", "c", "d", "e", "f", "g", "h",
                        //   "A", "B", "C", "D", "E", "F", "G", "H"];

const CHARS: [&str; 2] = ["a", "b"];

fn number_to_string(mut x: usize) -> String {
    let base = CHARS.len();
    let mut result = String::new();

    // Adjust `x` to be 0-indexed
    x -= 1;

    loop {
        let index = x % base;
        result = format!("{}{}", CHARS[index], result);
        if x < base {
            break;
        }
        x = x / base - 1;
    }

    format!(".{}", result)
}

pub struct Label {
    offset: usize,
    labels: HashMap<String, String>,
}
impl Label {
    pub fn new() -> Self {
        Self {
            offset: 0,
            labels: HashMap::new(),
        }
    }
    pub fn increment(&mut self) -> String {
        self.offset += 1;
        number_to_string(self.offset)
    }
    pub fn get(&mut self, key: String) -> Option<&String> {
        self.labels.get(&key)
    }
    pub fn generate(&mut self, key: &String) -> String {
        match self.labels.get(key) {
            Some(label) => label.to_owned(),
            None => {
                self.offset += 1;
                let value = number_to_string(self.offset);
                self.labels.insert(key.clone(), value.clone());
                return value
            }
        }
    }
}
