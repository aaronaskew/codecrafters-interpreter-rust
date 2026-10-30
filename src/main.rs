#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::exit;

use crate::parse::Parser;
use crate::scan::Scanner;

mod parse;
mod scan;
mod token;

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
            let mut parser = Parser::from(Scanner::from_str(&file_contents));
            parser.parse().print_ast();
        }
        "tokenize" => {
            let scanner = Scanner::from_str(&file_contents);

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
