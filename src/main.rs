#![feature(io, match_default_bindings, box_syntax)]

extern crate colored;
extern crate failure;

mod location;
mod token;
mod tokenizer;
mod node;
mod parser;

use tokenizer::Tokenizer;
use parser::Parser;

fn main() {
    let tokenizer = Tokenizer::new("scratch.bolt").unwrap();
    let mut parser = Parser::new(tokenizer);

    println!("{:#?}", parser.parse().unwrap());
}
