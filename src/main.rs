#![feature(io, match_default_bindings, box_syntax)]

extern crate colored;
extern crate failure;
extern crate xml;

mod location;
mod token;
mod tokenizer;
mod node;
mod parser;

use tokenizer::Tokenizer;
use parser::Parser;
use xml::EmitterConfig;
use std::io::{self, Write};
use node::WriteXml;

fn main() {
    let tokenizer = Tokenizer::new("scratch.bolt").unwrap();
    let mut parser = Parser::new(tokenizer);

    let node = parser.parse().unwrap();

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(io::stdout());
    node.write_xml(&mut writer).unwrap();

    println!();
}
