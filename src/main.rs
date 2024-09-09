use std::env;
use std::fs;
use std::io::{self, Write};

use scanner::Scanner;

pub mod scanner;

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

        match command.as_str() {
            "tokenize" => {
                // You can use print statements as follows for debugging, they'll be visible when running tests.
                writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

                let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                    String::new()
                });

                Self::run(file_contents.to_string());
            }
            _ => {
                writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
                return;
            }
        }

    }

    pub fn run(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

        for token in tokens {
            println!("{}", token);
        }

        if scanner.had_error {
            std::process::exit(65);
        }
    }
}