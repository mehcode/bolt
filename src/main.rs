#![feature(io, match_default_bindings, box_syntax)]

extern crate colored;
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
use std::io::{self, Write};
use ast::Serialize;

fn main() {
    let tokenizer = Tokenizer::new("scratch.bolt").unwrap();
    let mut parser = Parser::new(tokenizer);

    let node = parser.parse().unwrap();

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(io::stdout());

    node.serialize(&mut writer).unwrap();

    println!();
}
