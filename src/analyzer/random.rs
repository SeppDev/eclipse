use std::time::{SystemTime, UNIX_EPOCH};

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let a = (self.state >> 32) as u32;
        let max: u32 = 4294967295;
        let value = a as f64 / max as f64;

        return value
    }
    fn integer(&mut self, min: i32, max: i32) -> i32 {
        let float = self.next();
        let result = ((max as f64 - min as f64) * float).round() + min as f64;
        return result as i32;
    }
    fn bool(&mut self) -> bool {
        return self.next() > 0.5;
    }
}

fn random_bytes(length: usize) -> Vec<u8> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    let mut rand = SimpleRng::new(seed);
    let mut numbers = Vec::new();
    for _ in 0..length {
        if rand.bool() {
            numbers.push(rand.integer(97, 122) as u8);
        } else {
            numbers.push(rand.integer(65, 90) as u8);
        }
    }

    return numbers;
}