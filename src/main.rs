#![feature(io, match_default_bindings)]

extern crate colored;
extern crate failure;

mod location;
mod token;
mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new("./scratch.bolt").unwrap();

    while let Ok(Some(token)) = tokenizer.next() {
        println!("{:?}", token);
    }
}
