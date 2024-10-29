use std::time::{SystemTime, UNIX_EPOCH};

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

