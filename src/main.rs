use std::{env::args, path::PathBuf, io::{BufRead, Write}, fmt::Display};


fn main() {

    let args = args().collect::<Vec<String>>();

    if args.len() > 2 {
        eprintln!("Usage: lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        
        match run_file(&args[1]) {
            Ok(_) => (),
            Err(err) => err.error_with_exit() ,
        };
        
        
    } else {
        match run_prompt() {
            Ok(_) => (),
            Err(err) => err.error_with_exit(),
        };
    }

}

fn run_prompt() -> Result<(), LoxError> {
    
    show_prompt()?;
    
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(&line) {
                Ok(_) => (),
                Err(err) => err.report(),
            };
            show_prompt()?;
            
        } else {
            break;
        }
    }
    
    Ok(())
}
fn show_prompt() -> Result<(), LoxError> {

    print!("> ");
    match std::io::stdout().flush() {
        Ok(_) => Ok(()),
        Err(_) => Err(LoxError::new(0, "Could not display prompt")),
    }
}

fn run_file(path: impl Into<PathBuf>) -> Result<(), LoxError> {
    
    if let Ok(buf) = std::fs::read_to_string(path.into()) {
        match run(&buf) {
            Ok(_) => Ok(()),
            Err(err) => Ok(err.error_with_exit()),
        }
    }   else {
        Err(LoxError::new(0, "Could not read file"))
    }
    
}


fn run(source: &str) -> Result<(), LoxError> {
    
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

struct Scanner {
    source: String,
}

#[derive(Debug)]
struct Token {
    tok: String,
}

impl Scanner {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
        }
    }
}

impl Scanner {
    pub fn scan_tokens(&self) -> Result<Vec<Token>, LoxError> {
        Ok(self.source.split_whitespace().map(|s| Token::new(s)).collect())
    }
}

impl Token {
    pub fn new(tok: impl Into<String>) -> Self {
        Self {
            tok: tok.into(),
        }
    }
}


struct LoxError {
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