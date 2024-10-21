use std::env;
use std::fs;
use std::io::{self, Write};

use interpreter::Interpreter;
use parser::expr::AstPrinter;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;

pub mod scanner;
pub mod utils;
pub mod parser;
pub mod error;
pub mod interpreter;
pub mod statement;
pub mod resolver;

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

        match command.as_str() {
            "tokenize" => {
                let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                    String::new()
                });

                Self::tokenize(file_contents.to_string());
            },
            "parse" => {
                let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                    String::new()
                });
                
                Self::parse(file_contents.to_string())
            },
            "evaluate" => {
                let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                    String::new()
                });
                
                Self::evaluate(file_contents.to_string())

            },
            "run" => {
                let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                    String::new()
                });
                
                Self::run(file_contents.to_string())
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

        if scanner.had_error {
            std::process::exit(65);
        }

        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        if expression.is_none() {
            std::process::exit(65);
        }

        if let Some(e) = expression {
            println!("{}", AstPrinter::print(e));
        }
    }

    pub fn evaluate(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

        if scanner.had_error {
            std::process::exit(65);
        }

        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        if expression.is_none() {
            std::process::exit(65);
        }

        let mut interpreter = Interpreter::new();

        let v = interpreter.interpret(expression.unwrap());

        if v.is_none() {
            std::process::exit(70);
        }

        let v = v.unwrap();

        println!("{}", v);
    }

    pub fn run(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

        if scanner.had_error {
            std::process::exit(65);
        }

        let mut parser = Parser::new(tokens);

        let statements = parser.parse_statement();



        match statements {
            Ok(statements) => {
                let mut interpreter = Interpreter::new();
                let mut resolver = Resolver::new(interpreter);

                let r = resolver.resolve_statements(statements.clone());

                if let Err(_) = r {
                    // println!("{:?}", resolver.scopes);
                    std::process::exit(65);
                }

                eprintln!("Resolving complete, now interpreting");

                interpreter = resolver.interpreter;                
                interpreter.interpret_statements(statements);
            },
            Err(_) => {std::process::exit(65);}
        }
    }



    
}

fn char_at(string: &str, n: usize) -> char {
    return string.as_bytes()[n] as char;
}

