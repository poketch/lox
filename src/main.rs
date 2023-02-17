mod token;
mod scanner;
use scanner::Scanner;

mod error;
use error::LoxError;

use std::{env::args, path::PathBuf, io::{BufRead, Write}};

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
    
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens()?;
    

    for token in scanner.tokens() {
        println!("{}", token);
    }

    Ok(())
}
