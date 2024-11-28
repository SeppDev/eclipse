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

type Status = Option<String>;

#[derive(Debug)]
pub struct CompileCtx {
    debuginfo: MsgMap,
    messages: Vec<String>,

    lines: HashMap<Path, Vec<String>>,
    current_file_path: Path,

    sender: Sender<Status>,
    done: Receiver<()>,
}
impl CompileCtx {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<Status>();
        let (done_sender, done) = mpsc::channel();

        std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let mut message = String::new();
            loop {
                match receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(m) => match m {
                        Some(m) => message = m,
                        None => break
                    },
                    Err(_) => {},
                };
                print!("\r\x1b[2K({:?}s) - {}", start.elapsed().as_secs(), message);
                let _ = std::io::stdout().flush();
            }

            // Clearing output
            print!("\r\x1b[2K");
            let _ = std::io::stdout().flush();
            let _ = done_sender.send(());
        });


        Self {
            debuginfo: MsgMap::default(),
            messages: Vec::new(),
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
        exit(1)
    }
    pub fn push(&mut self, relative_file_path: Path, message: Message) {
        match &message.kind {
            MessageKind::Note => self.debuginfo.errors.push((relative_file_path, message)),
            MessageKind::Warning => self.debuginfo.warnings.push((relative_file_path, message)),
            MessageKind::Error => self.debuginfo.errors.push((relative_file_path, message)),
        }
    }
    pub fn set_path(&mut self, path: &Path) {
        self.current_file_path = path.clone()
    }

    pub fn set_status<T: ToString>(&self, message: T) {
        let _ = self.sender.send(Some(message.to_string()));
    }
    pub fn result_print<T: ToString>(&mut self, message: T) {
        self.messages.push(message.to_string());
    }

    pub fn finish(&self) {
        let _ = self.sender.send(None);
        let _ = self.done.recv_timeout(Duration::from_secs(5));
    }
    pub fn throw(&self, finish: bool) {
        let has_errors = self.debuginfo.errors.len() > 0;
        if !has_errors && !finish {
            return;
        }

        self.finish();
        
        self.display(&self.debuginfo.notes);
        self.display(&self.debuginfo.warnings);
        self.display(&self.debuginfo.errors);

        for msg in &self.messages {
            println!("{}", msg);
        }
        
        if has_errors {
            exit(1)
        }
    }

    pub fn error<T: ToString>(&mut self, location: Location, message: T) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.debuginfo
            .errors
            .push((self.current_file_path.clone(), message));
        return self.debuginfo.errors.last_mut().unwrap().1.borrow_mut();
    }
    pub fn warning<T: ToString>(&mut self, location: Location, message: T) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.debuginfo
            .errors
            .push((self.current_file_path.clone(), message));
        return self.debuginfo.warnings.last_mut().unwrap().1.borrow_mut();
    }
    pub fn note<T: ToString>(&mut self, location: Location, message: T) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.debuginfo
            .errors
            .push((self.current_file_path.clone(), message));
        return self.debuginfo.notes.last_mut().unwrap().1.borrow_mut();
    }
}
