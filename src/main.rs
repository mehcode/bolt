#![feature(io)]

extern crate colored;
#[macro_use]
extern crate failure;
extern crate xml;

mod location;
mod token;
mod tokenizer;
mod parser;
mod ast;
mod symbol;

use tokenizer::Tokenizer;
use parser::Parser;
use xml::EmitterConfig;
use std::io;
use ast::Serialize;
use std::env;

fn main() {
    let argv: Vec<_> = env::args().collect();

    let tokenizer = Tokenizer::new(&argv[1]).unwrap();
    let mut parser = Parser::new(tokenizer);

    let node = parser.parse().unwrap();

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(io::stdout());

    node.serialize(&mut writer).unwrap();

    println!();
}
