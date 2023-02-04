use std::io::{self, Write};

pub fn start() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.trim() == "exit" {
            break println!("bye!");
        }

        println!("{}", line.trim());
    }
}
