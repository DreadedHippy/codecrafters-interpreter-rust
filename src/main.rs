use std::env;
use std::fs;
use std::io::{self, Write};

use parser::expr::AstPrinter;
use parser::Parser;
use scanner::Scanner;

pub mod scanner;
pub mod utils;
pub mod parser;
pub mod error;

fn main() {
    let args: Vec<String> = env::args().collect();
    Lox::main(args);
}


pub struct Lox;

impl Lox { 
    pub fn main(args: Vec<String>){
        
        if args.len() < 3 {
            writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
            return;
        }

        let command = &args[1];
        let filename = &args[2];

        
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

        let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
            String::new()
        });

        match command.as_str() {
            "tokenize" => {
                Self::tokenize(file_contents.to_string());
            },
            "parse" => {
                Self::parse(file_contents.to_string())
            }
            _ => {
                writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
                return;
            }
        }

    }

    pub fn tokenize(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

        for token in tokens {
            println!("{}", token);
        }

        if scanner.had_error {
            std::process::exit(65);
        }
    }

    pub fn parse(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        if let Some(e) = expression {
            println!("{}", AstPrinter::print(e));
        }
    }

    
}

fn char_at(string: &str, n: usize) -> char {
    return string.as_bytes()[n] as char;
}

