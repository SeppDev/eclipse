use std::{
    io::Write,
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};

// impl CompilerCtx {
//     pub fn message(&self, message: &str) {
//         if let Some(status) = &self.status {
//             status.message(message.into());
//         }
//     }
// }

pub struct Status {
    sender: Sender<Option<String>>,
    done_receiver: Receiver<()>,
}
impl Status {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<Option<String>>();
        let (done_sender, done_receiver) = mpsc::channel::<()>();

        let start = std::time::Instant::now();

        std::thread::spawn(move || {
            let mut message = String::new();
            loop {
                match receiver.recv_timeout(Duration::from_millis(200)) {
                    Ok(m) => match m {
                        Some(m) => message = m,
                        None => break,
                    },
                    Err(_) => {}
                };
                print!("\r\x1b[2K({:?}s) - {}", start.elapsed().as_secs(), message);
                let _ = std::io::stdout().flush();
            }

            print!("\r\x1b[2K");
            let _ = std::io::stdout().flush();
            let _ = done_sender.send(());
        });

        let _ = sender.send(Some("".to_string()));

        Self {
            sender,
            done_receiver,
        }
    }
    pub fn message(&self, message: String) {
        self.sender.send(Some(message)).unwrap();
    }
    pub fn quit(&self) {
        let _ = self.sender.send(None);
        let _ = self.done_receiver.recv_timeout(Duration::from_secs(2));
    }
}

impl Drop for Status {
    fn drop(&mut self) {
        self.quit();
    }
}
