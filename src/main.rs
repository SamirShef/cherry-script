use std::io::Read;

pub mod compiler;
use compiler::lexer::Lexer;

mod vm;
use vm::{
    VM,
    opcodes::OpCode,
    chunk::Chunk,
    stack_slot::StackSlot
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if (args.len() != 2) {
        println!("Error: Usage: cherry path/to/file.sd");
    }
    let file = std::fs::File::open(&args[1]);
    let mut content = String::new();
    match file {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut content);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    let mut  lexer = Lexer::new(content);
    while let token = lexer.next_token() && token != None {
        println!("{:?}", token.unwrap());
    }
}