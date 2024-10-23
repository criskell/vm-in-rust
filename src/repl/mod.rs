use crate::vm::VM;
use std::io::Write;
use std::{io, process};

/// REPL is:
/// - Read
/// - Evaluate
/// - Print
/// - Loop
pub struct REPL {
    command_history: Vec<String>,

    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_history: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");

        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!(">>> ");

            // `print` does not flush output automatically like `println`.
            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            let buffer = buffer.trim();

            self.command_history.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Goodbyte!");
                    process::exit(0);
                }

                ".history" => {
                    for command in &self.command_history {
                        println!("{}", command);
                    }
                }

                _ => {
                    println!("Invalid input!");
                }
            }
        }
    }
}
