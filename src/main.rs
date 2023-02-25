mod scanner;
use scanner::Scanner;

mod token;

mod error;
use error::LoxError;

mod expression;
mod keywords;

mod ast_printer;

use std::{
    env::args,
    io::{BufRead, Write},
    path::PathBuf,
};

fn main() -> Result<(), LoxError> {

    // // -1 * (234)
    // let minus = token::token::new(token::tokentype::minus, "-", none, 0); 
    // let one = expression::literalexpr::new(some(token::object::num(1.)));
    // let minus_one = expression::unaryexpr::new(minus, expression::expr::literal(one));

    // let times = token::token::new(token::tokentype::star, "*", none, 0);

    // let twothreefour = expression::literalexpr::new(some(token::object::num(234.)));
    // let twothreefour = expression::groupingexpr::new(expression::expr::literal(twothreefour));

    // let be = expression::binaryexpr::new(expression::expr::unary(minus_one), times, expression::expr::grouping(twothreefour));

    // let printer = ast_printer::astprinter::new();

    // println!("{}", printer.print(&expression::expr::binary(be))?); 

    let args = args().collect::<Vec<String>>();

    if args.len() > 2 {
        eprintln!("usage: lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => (),
            Err(err) => err.error_with_exit(),
        };
    } else {
        match run_prompt() {
            Ok(_) => (),
            Err(err) => err.error_with_exit(),
        };
    }

    Ok(())
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
        Err(_) => Err(LoxError::new(0, "could not display prompt")),
    }
}

fn run_file(path: impl Into<PathBuf>) -> Result<(), LoxError> {
    if let Ok(buf) = std::fs::read_to_string(path.into()) {
        match run(&buf) {
            Ok(_) => Ok(()),
            Err(err) => Ok(err.error_with_exit()),
        }
    } else {
        Err(LoxError::new(0, "could not read file"))
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
