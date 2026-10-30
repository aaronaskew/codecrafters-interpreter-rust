#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::exit;

use lox_interpreter::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "parse" => {
            let mut parser = Parser::from(file_contents.as_str());
            //    parser.parse().print_ast();
        }
        "tokenize" => {
            let scanner = Scanner::from(file_contents.as_str());

            print!("{}", scanner);

            if scanner.has_error {
                exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
