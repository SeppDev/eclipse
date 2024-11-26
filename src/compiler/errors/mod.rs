mod message;

use super::path::Path;
pub use message::{Message, MessageKind};
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    io::Write,
    ops::Range,
    process::exit,
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};

pub type CompileResult<T> = Result<T, ()>;

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub lines: Range<usize>,
    pub columns: Range<usize>,
}
impl Location {
    pub fn new(lines: Range<usize>, columns: Range<usize>) -> Self {
        Self { lines, columns }
    }
    pub fn single(line: usize, column: usize) -> Self {
        Self {
            lines: line..line,
            columns: column..column,
        }
    }
    pub fn void() -> Self {
        Self::single(0, 0)
    }
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "columns: {}-{}, lines: {}-{}",
            self.columns.start, self.columns.end, self.lines.start, self.lines.end
        )
    }
}

type Map = Vec<(Path, Message)>;

#[derive(Debug, Default)]
struct MsgMap {
    notes: Map,
    warnings: Map,
    errors: Map,
}

#[derive(Debug)]
pub struct CompileCtx {
    messages: MsgMap,
    lines: HashMap<Path, Vec<String>>,
    current_file_path: Path,

    sender: Sender<Option<String>>,
    done: Receiver<()>,
}
impl CompileCtx {
    pub fn new() -> Self {
        let (sender, receiver): (Sender<Option<String>>, Receiver<Option<String>>) =
            mpsc::channel();

        let (done_sender, done) = mpsc::channel();

        let start = std::time::Instant::now();

        std::thread::spawn(move || {
            let mut message = String::new();
            loop {
                match receiver.recv() {
                    Ok(m) => match m {
                        Some(a) => {
                            if a.len() > 0 {
                                message = a
                            }
                        }
                        None => break,
                    },
                    Err(_) => break,
                };
                print!("\r({:?}s) - {}\r", start.elapsed().as_secs(), message);
                std::io::stdout().flush().unwrap();
            }

            // Clearing output
            print!("\r");
            std::io::stdout().flush().unwrap();
            done_sender.send(()).unwrap();
        });

        let tick = sender.clone();
        
        #[allow(unused_must_use)]
        std::thread::spawn(move || loop {
            tick.send(Some(String::new()));
            std::thread::sleep(Duration::from_millis(1));
        });

        Self {
            messages: MsgMap::default(),
            lines: HashMap::new(),
            current_file_path: Path::default(),

            sender,
            done,
        }
    }
    pub fn set_lines(&mut self, relative_path: Path, lines: Vec<String>) {
        self.lines.insert(relative_path, lines);
    }
    pub fn quit(&self) -> ! {
        self.throw(true);
        println!("No debuginfo found, but quitted");
        exit(1)
    }
    pub fn push(&mut self, relative_file_path: Path, message: Message) {
        match &message.kind {
            MessageKind::Note => self.messages.errors.push((relative_file_path, message)),
            MessageKind::Warning => self.messages.warnings.push((relative_file_path, message)),
            MessageKind::Error => self.messages.errors.push((relative_file_path, message)),
        }
    }
    pub fn set_path(&mut self, path: &Path) {
        self.current_file_path = path.clone()
    }
    pub fn set_status<T: ToString>(&self, message: T) {
        self.sender.send(Some(message.to_string())).unwrap();
    }

    #[allow(unused_must_use)]
    pub fn finish(&self) {
        self.sender.send(None);
        self.done.recv_timeout(Duration::from_secs(1));
    }
    pub fn throw(&self, finish: bool) {
        let has_errors = self.messages.errors.len() > 0;
        if !has_errors && !finish {
            return;
        }
        self.finish();

        self.display(&self.messages.notes);
        self.display(&self.messages.warnings);
        self.display(&self.messages.errors);

        if has_errors {
            exit(1)
        }
    }

    pub fn error<T: ToString>(&mut self, location: Location, message: T) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.messages
            .errors
            .push((self.current_file_path.clone(), message));
        return self.messages.errors.last_mut().unwrap().1.borrow_mut();
    }
    pub fn warning<T: ToString>(&mut self, location: Location, message: T) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.messages
            .errors
            .push((self.current_file_path.clone(), message));
        return self.messages.warnings.last_mut().unwrap().1.borrow_mut();
    }
    pub fn note<T: ToString>(&mut self, location: Location, message: T) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.messages
            .errors
            .push((self.current_file_path.clone(), message));
        return self.messages.notes.last_mut().unwrap().1.borrow_mut();
    }
}
