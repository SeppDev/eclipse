pub struct Command<'a> {
    long: &'a str,
    short: Option<char>,
    help: Option<&'a str>,
}
impl<'a> Command<'a> {
    pub fn new(long: &'a str) -> Self {
        Self {
            long,
            short: None,
            help: None,
        }
    }
    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }
    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
}
