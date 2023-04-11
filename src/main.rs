mod chunk;
mod compiler;
mod opcode;
mod precedence;
mod scanner;
mod token;
mod token_type;
mod value;
mod vm;

use crate::opcode::OpCode;
use crate::vm::*;
use std::env::args;
use std::io;
use std::io::{stdout, Write};

fn main() {
    let args: Vec<String> = args().collect();
    let mut vm = VM::new();
    match args.len() {
        1 => {
            repl(&mut vm);
        }
        2 => {
            run_file(&mut vm, &args[1]).expect("Error: something is wrong");
        }
        _ => {
            println!("Usage: bytecode-lox [script]");
            std::process::exit(64);
        }
    }
    vm.free();
}

fn repl(vm: &mut VM) {
    let stdin = io::stdin();
    print!("> ");
    let _ = stdout().flush();
    for line in stdin.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                println!("Please enter something to execute");
                print!("> ");
                let _ = stdout().flush();
                continue;
            }

            let _ = vm.interpret(&line);
        } else {
            break;
        }
        print!("> ");
        let _ = stdout().flush();
    }
}
fn run_file(vm: &mut VM, path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    match vm.interpret(&buf) {
        Err(InterpretResult::RuntimeError) => std::process::exit(66),
        Err(InterpretResult::CompileError) => std::process::exit(65),
        Ok(_) => std::process::exit(0),
    }
}
