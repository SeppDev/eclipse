const CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const CHARS_LEN: usize = CHARS.len();

#[derive(Debug, Default)]
pub struct NameCounter {
    count: usize,
}
impl NameCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
    pub fn increment(&mut self) -> String {
        let mut string = String::with_capacity(8);
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

        string
    }
}
