mod ast;
mod emitter;
mod lexer;
mod parser;
mod relp;
mod token;

fn main() {
    relp::start();
}
