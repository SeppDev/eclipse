mod message;

use super::{codegen::target::Target, counter::NameCounter, path::Path};
pub use message::CompileMessage;
use message::MessageVariant;
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    io::Write,
    ops::Range,
    path::PathBuf,
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
    pub fn set_start(&mut self, location: &Location) {
        self.columns.start = location.columns.start;
        self.lines.start = location.lines.start;
    }
    pub fn set_end(&mut self, location: &Location) {
        self.columns.end = location.columns.end;
        self.lines.end = location.lines.end;
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

type Map = Vec<(Path, CompileMessage)>;

#[derive(Debug, Default)]
struct MsgMap {
    warnings: Map,
    errors: Map,
}
type Status = Option<String>;

pub struct CompileCtx {
    pub counter: NameCounter,
    pub project_dir: PathBuf,
    pub target: Target,
    pub current_file_path: Path,

    debuginfo: MsgMap,
    messages: Vec<String>,

    lines: HashMap<Path, Vec<String>>,

    sender: Sender<Status>,
    done: Receiver<()>,
}
impl CompileCtx {
    pub fn new(project_dir: PathBuf) -> Self {
        let (sender, receiver) = mpsc::channel::<Status>();

        let (done_sender, done) = mpsc::channel();

        let start = std::time::Instant::now();

        std::thread::spawn(move || {
            let mut message = String::new();
            loop {
                match receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(m) => match m {
                        Some(m) => message = m,
                        None => break,
                    },
                    Err(_) => {}
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
            target: Target::new(),
            counter: NameCounter::new(),
            project_dir,

            debuginfo: MsgMap::default(),
            messages: Vec::new(),
            lines: HashMap::new(),
            current_file_path: Path::default(),

            sender,
            done,
        }
    }
    pub fn set_lines(&mut self, lines: Vec<String>) {
        self.lines.insert(self.current_file_path.clone(), lines);
    }
    pub fn quit(&self) -> ! {
        self.throw(true);
        println!("No debuginfo found, but quitted");
        exit(1)
    }
    pub fn push(&mut self, relative_file_path: Path, message: CompileMessage) {
        match &message.variant {
            MessageVariant::Warning => self.debuginfo.warnings.push((relative_file_path, message)),
            MessageVariant::Error => self.debuginfo.errors.push((relative_file_path, message)),
        }
    }
    pub fn set_current_path(&mut self, path: Path) {
        self.current_file_path = path
    }

    pub fn set_status<T: ToString>(&self, message: T) {
        let _ = self.sender.send(Some(message.to_string()));
    }
    pub fn result_print<T: ToString>(&mut self, message: T) {
        self.messages.push(message.to_string());
    }
    fn stop_status(&self) {
        let _ = self.sender.send(None);
        let _ = self.done.recv_timeout(Duration::from_secs(1));
    }
    pub fn throw(&self, finish: bool) {
        let has_errors = self.debuginfo.errors.len() > 0;
        if !has_errors && !finish {
            return;
        }
        self.stop_status();

        self.display(&self.debuginfo.warnings);
        self.display(&self.debuginfo.errors);

        for msg in &self.messages {
            println!("{}", msg);
        }

        if has_errors {
            exit(1)
        }
    }

    pub fn error<T: ToString>(&mut self, location: Location, message: T) -> &mut CompileMessage {
        let mut message = CompileMessage::error(message.to_string());
        message.push("", location);
        self.debuginfo
            .errors
            .push((self.current_file_path.clone(), message));
        return self.debuginfo.errors.last_mut().unwrap().1.borrow_mut();
    }
    pub fn warning<T: ToString>(&mut self, location: Location, message: T) -> &mut CompileMessage {
        let mut message = CompileMessage::warning(message.to_string());
        message.push("", location);
        self.debuginfo
            .warnings
            .push((self.current_file_path.clone(), message));
        return self.debuginfo.warnings.last_mut().unwrap().1.borrow_mut();
    }
}
