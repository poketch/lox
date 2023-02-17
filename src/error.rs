use std::fmt::Display;

pub struct LoxError {
    line: usize,
    msg: String, 
}

impl LoxError {

    pub fn new(line: usize, msg: impl Into<String>) -> Self {

        Self {
            line,
            msg: msg.into(),
        }

    }

    pub fn report(&self) -> () {

        println!("{}", self);
    }

    pub fn error_with_exit(&self) -> () {
        self.report();
        std::process::exit(65);
    }


}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Line {}] Error: {}", self.line, self.msg)
    }
}