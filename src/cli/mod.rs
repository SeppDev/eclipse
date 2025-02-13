use arguments::{Argument, Arguments};

use crate::common::exit::exit;
pub mod arguments;

type Handler = dyn FnOnce(Arguments) + 'static;

pub struct CLI {
    arguments: Arguments,
    commands: Vec<Command>,
}
pub struct Command {
    description: String,
    aliases: Vec<String>,
    handler: Box<Handler>,
}
impl CLI {
    pub fn new() -> Self {
        Self {
            arguments: Arguments::new(),
            commands: Vec::new(),
        }
    }
    fn help(self) -> ! {
        let mut length: usize = 0;
        let mut commands = Vec::new();
        
        for command in &self.commands {
            let name = command.aliases.join(", ");
            length = length.max(name.len());
            commands.push(name);
        }
        length += 2;
        
        println!("Commands:");
        for (command, mut name) in self.commands.into_iter().zip(commands) {
            for _ in 0..length - name.len() {
                name.push(' ');
            }
            println!("\t{} {}", name, command.description);
        }
        
        std::process::exit(0);
    }
    pub fn next_argument(&mut self) -> Option<Argument> {
        self.arguments.next_argument()
    }
    pub fn start(mut self) {
        let command = match self.next_argument() {
            Some(a) => a,
            None => self.help(),
        };
        let argument = if let Argument::Value(s) = command {
            s
        } else {
            exit("Expected command got key, value")
        };
        if argument == "help" {
            self.help();
        }

        let command = match self.commands.into_iter().find_map(|command| {
            command
                .aliases
                .iter()
                .find(|c| c == &&argument)
                .is_some()
                .then_some(command)
        }) {
            Some(c) => c,
            None => exit(format!("No handler found for '{argument}'")),
        };

        (command.handler)(self.arguments)
    }
    pub fn register<F: FnOnce(Arguments) + 'static>(
        mut self,
        description: &str,
        aliases: Vec<&'static str>,
        handler: F,
    ) -> Self {
        let command = Command {
            description: description.to_string(),
            aliases: aliases.into_iter().map(|k| k.to_string()).collect(),
            handler: Box::new(handler),
        };

        self.commands.push(command);
        self
    }
}
