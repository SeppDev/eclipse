use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

#[derive(Debug)]
pub struct Random {
    state: usize,
}
impl Random {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as usize;

        Self { state: seed }
    }

    pub fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let a = (self.state >> 32) as u32;
        let max: u32 = 4294967295;
        let value = a as f64 / max as f64;

        return value;
    }
    pub fn integer(&mut self, min: i32, max: i32) -> i32 {
        let float = self.next();
        let result = ((max as f64 - min as f64) * float).round() + min as f64;
        return result as i32;
    }
    pub fn bool(&mut self) -> bool {
        return self.next() > 0.5;
    }
}

#[derive(Debug)]
pub struct RandomString {
    random: Random,
    generated: HashMap<String, bool>,
}
impl RandomString {
    pub fn new() -> Self {
        Self {
            random: Random::new(),
            generated: HashMap::new(),
        }
    }
    pub fn generate(&mut self) -> String {
        self.g(6)
    }
    fn g(&mut self, length: u32) -> String {
        let mut numbers = Vec::new();

        for _ in 0..length {
            if self.random.bool() {
                numbers.push(self.random.integer(97, 122) as u8);
            } else {
                numbers.push(self.random.integer(65, 90) as u8);
            }
        }

        let value = String::from_utf8(numbers).unwrap();
        match self.generated.insert(value.clone(), true) {
            Some(_) => self.g(length + 1),
            None => value,
        }
    }
}
