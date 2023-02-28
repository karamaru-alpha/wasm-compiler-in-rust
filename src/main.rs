mod ast;
mod emitter;
mod lexer;
mod parser;
mod token;

use std::{fs, io::prelude::*};


fn main() {
    let input = fs::read_to_string("a.txt").unwrap();

    let lexer = lexer::Lexer::new(&input);
    let mut parser = parser::Parser::new(lexer);
    let mut emitter = emitter::Emitter::new(parser.parse_program());
    let wasm =  emitter.emit();

    let mut file = fs::File::create("a.wasm").expect("err file create");
    file.write_all(&wasm).expect("err write file");
    println!("Wasm binary output to a.wasm.");
    println!("Binary: {}", wasm.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "));
    println!("Run `python3 -m http.server` (Edit index.html if necessary).");
}
