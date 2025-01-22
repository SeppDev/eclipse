use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::Instant,
};

pub struct Logger {
    time: Instant,
    file: fs::File,
}
impl Logger {
    pub fn new(name: &str) -> Self {
        let mut path = PathBuf::from("/tmp");
        path.push(name);
        path.set_extension("log");

        let time = Instant::now();
        let file = File::create(path).unwrap();

        Self { file, time }
    }
    pub fn write<Content: ToString>(&mut self, content: Content) {
        let time = self.time.elapsed().as_secs_f32();

        let message = format!("[{time:.1}s]: {}\n", content.to_string());
        self.file.write(message[..].as_bytes()).unwrap();
    }
}
