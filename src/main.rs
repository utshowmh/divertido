mod frontend;
mod general;
mod runtime;

use std::{
    env::args,
    fs::read_to_string,
    io::{stdin, stdout, Write},
    process::exit,
};

use general::error::Error;

use crate::{
    frontend::{lexer::Lexer, parser::Parser},
    runtime::interpreter::Interpreter,
};

fn main() {
    run().unwrap_or_else(|error| error.throw());
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => {
            let mut line = String::new();
            let stdin = stdin();
            let mut stdout = stdout();

            loop {
                print!("divertido :> ");
                stdout.flush().unwrap();
                stdin.read_line(&mut line).unwrap();

                let mut lexer = Lexer::new(&line);
                let tokens = lexer.lex()?;
                let mut parser = Parser::new(tokens);
                let statements = parser.parse()?;
                let mut interpreter = Interpreter::new();
                interpreter.run(statements)?;

                line.clear();
            }
        }
        2 => match args[1].as_str() {
            "repl" => {
                let mut line = String::new();
                let stdin = stdin();
                let mut stdout = stdout();

                loop {
                    print!("divertido :> ");
                    stdout.flush().unwrap();
                    stdin.read_line(&mut line).unwrap();

                    let mut lexer = Lexer::new(&line);
                    let tokens = lexer.lex()?;
                    let mut parser = Parser::new(tokens);
                    let statements = parser.parse()?;
                    let mut interpreter = Interpreter::new();
                    interpreter.run(statements)?;

                    line.clear();
                }
            }

            "help" => print_help(None),

            filepath => {
                if let Ok(source) = read_to_string(filepath) {
                    let mut lexer = Lexer::new(&source);
                    let tokens = lexer.lex()?;
                    let mut parser = Parser::new(tokens);
                    let statements = parser.parse()?;
                    let mut interpreter = Interpreter::new();
                    interpreter.run(statements)?;
                } else {
                    print_help(Some(&format!("Could not open file '{}'", filepath)))
                }
            }
        },
        _ => {
            print_help(Some("Invalid number of commands"));
        }
    }

    Ok(())
}

fn print_help(error: Option<&str>) {
    if let Some(message) = error {
        eprintln!("Error    :   {}.", message);
        eprintln!();
        eprintln!("Program  :   Divertido");
        eprintln!("Usage    :   divertido [command]");
        eprintln!("Command  :");
        eprintln!("    repl     :   runs a divertido repl.");
        eprintln!("    filename :   runs the given file.");
        eprintln!("    help     :   prints this page.");
        exit(1);
    } else {
        println!("Program: Divertido");
        println!("Usage: divertido [command]");
        println!("Command:");
        println!("    repl:       runs a divertido repl.");
        println!("    filename:   runs the given file.");
        exit(0);
    }
}
