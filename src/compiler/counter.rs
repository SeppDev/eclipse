const CHARS: [char; 16] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
];
const CHARS_LEN: usize = CHARS.len();

#[derive(Debug)]
pub struct NameCounter {
    count: usize,
}
impl NameCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
    pub fn increment(&mut self) -> String {
        let mut string = String::new();
        let mut count = self.count;
        self.count += 1;

        loop {
            string.push(CHARS[count % CHARS_LEN]);

            if count >= CHARS_LEN {
                count /= CHARS_LEN
            } else {
                break;
            }
        }

        return string;
    }
}
