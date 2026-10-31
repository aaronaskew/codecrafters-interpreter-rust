#![allow(unused_variables)]
use clap::Parser;
use clap::Subcommand;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use codecrafters_interpreter::*;

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
}

fn file_contents(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Failed to read file {:?}", path);
        String::new()
    })
}

fn main() -> anyhow::Result<()> {
    // TODO: use clap

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Tokenize { file_path } => {
            let source = file_contents(file_path);
            let scanner = Scanner::from(source.as_str());

            print!("{}", scanner);

            if scanner.has_error {
                exit(65);
            }
        }
        Commands::Parse { file_path } => {
            let source = file_contents(file_path);
            let scanner = Scanner::from(source.as_str());
            let mut parser = codecrafters_interpreter::Parser::new(&scanner.tokens());

            match parser.parse() {
                Ok(expr) => expr.print_ast(),
                Err(e) => exit(65),
            }
        }
        Commands::Evaluate { file_path } => {
            let source = file_contents(file_path);
            let scanner = Scanner::from(source.as_str());
            let mut parser = codecrafters_interpreter::Parser::new(&scanner.tokens());

            match parser.parse() {
                Ok(expr) => {
                    println!("{}", Interpreter::new(expr).evaluate())
                }
                Err(e) => exit(65),
            }
        }
    }

    Ok(())
}
