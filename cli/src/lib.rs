use command::Command;

pub mod command;

pub struct CLI<'a> {
    name: &'a str,
    commands: Vec<Command<'a>>,
}
impl<'a> CLI<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            commands: Vec::new(),
        }
        .command(Command::new("help").short('h'))
    }
    pub fn command(mut self, command: Command<'a>) -> Self {
        self.commands.push(command);
        self
    }
    pub fn start(self) {
        println!("test")
    }
}

// env!("CARGO_PKG_VERSION")
