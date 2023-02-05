use crate::{lexer::Lexer, parser::Parser};
use std::io::{self, Write};

pub fn start() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let input = line.trim();
        if input == "exit" {
            break println!("bye!");
        }

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        println!("{:?}", program.statements);
    }
}
