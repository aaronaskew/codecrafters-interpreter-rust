#![allow(unused_variables)]
use clap::Parser;
use clap::Subcommand;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use lox_interpreter::*;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tokenize { file_path: PathBuf },
    Parse { file_path: PathBuf },
    Evaluate { file_path: PathBuf },
    Run { file_path: PathBuf },
}

fn file_contents(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Failed to read file {:?}", path);
        String::new()
    })
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Tokenize { file_path } => {
            let source = file_contents(file_path);
            let scanner = Scanner::from(source.as_str());

            print!("{}", scanner);

            if scanner.has_error {
                exit(SYNTAX_ERROR_EXIT_CODE);
            }
        }
        Commands::Parse { file_path } => {
            let source = file_contents(file_path);
            let scanner = Scanner::from(source.as_str());
            let mut parser = lox_interpreter::Parser::new(&scanner.tokens());

            parser.parse_expr().print_ast();
        }
        Commands::Evaluate { file_path } => {
            let source = file_contents(file_path);
            let scanner = Scanner::from(source.as_str());
            let mut parser = lox_interpreter::Parser::new(&scanner.tokens());
            let expr = parser.parse_expr();
            let mut interpreter = Interpreter::new();

            println!(
                "{}",
                interpreter
                    .evaluate(&expr)
                    .expect("expression should evaluate without error")
            )
        }
        Commands::Run { file_path } => {
            let source = file_contents(file_path);
            let scanner = Scanner::from(source.as_str());
            let mut parser = lox_interpreter::Parser::new(&scanner.tokens());
            let statements = parser.parse();
            let mut interpreter = Interpreter::new();
            interpreter.interpret(statements);
        }
    }
}
