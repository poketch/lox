use std::{env::args, path::PathBuf, io::{BufRead, Write}};


fn main() {

    let args = args().collect::<Vec<String>>();

    if args.len() > 2 {
        eprintln!("Usage: lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt().expect("Could not run prompt");
    }

}

fn run_prompt() -> Result<(), std::io::Error> {
    
    show_prompt()?;
    
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            run(&line)?;
            show_prompt()?;
            
        } else {
            break;
        }
    }
    
    Ok(())
}
fn show_prompt() -> Result<(), std::io::Error> {

    print!("> ");
    std::io::stdout().flush()?;
    Ok(())
}

fn run_file(path: impl Into<PathBuf>) -> Result<(), std::io::Error> {
    let buf = std::fs::read_to_string(path.into())?;
    run(&buf)?;
    Ok(())
    
}


fn run(source: &str) -> Result<(), std::io::Error> {
    
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    

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
    pub fn scan_tokens(&self) -> Vec<Token> {
        self.source.split_whitespace().map(|s| Token::new(s)).collect()
    }
}

impl Token {
    pub fn new(tok: impl Into<String>) -> Self {
        Token {
            tok: tok.into(),
        }
    }
}