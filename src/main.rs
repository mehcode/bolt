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
mod log;

use tokenizer::Tokenizer;
use parser::Parser;
use xml::EmitterConfig;
use std::io;
use ast::Serialize;
use std::env;
use failure::Error;
use log::Log;

fn main() {
    match run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}

fn run() -> Result<(), Error> {
    let argv: Vec<_> = env::args().collect();

    let log = Log::new();

    let tokenizer = Tokenizer::new(&argv[1], &log)?;
    let mut parser = Parser::new(tokenizer);

    let node = parser.parse()?;

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(io::stdout());

    node.serialize(&mut writer)?;

    println!();

    Ok(())
}
