#![feature(io, match_default_bindings)]

extern crate colored;
extern crate failure;

use std::fs::File;
use std::iter::Peekable;
use std::io::{Read, Chars};
use failure::Error;
use colored::*;

#[derive(Debug)]
enum Token {
    Identifier { text: String },
    Integer { value: String, radix: u8 },
}

struct Tokenizer {
    stream: Peekable<Chars<File>>,
}

macro_rules! try_peek {
    ($e:expr) => (
        match $e.peek() {
            Some(Ok(v)) => *v,
            _ => return None,
        }
    )
}

macro_rules! try_next {
    ($e:expr) => (
        match $e.next() {
            Some(Ok(v)) => v,
            Some(Err(err)) => return Some(Err(err.into())),
            None => return None,
        }
    )
}

impl Tokenizer {
    fn new(filename: &str) -> Result<Self, Error> {
        let stream = File::open(filename)?.chars().peekable();

        Ok(Self { stream })
    }

    fn next(&mut self) -> Option<Result<Token, Error>> {
        // Consume whitespace
        while let Some(Ok(ch)) = self.stream.peek() {
            if ch.is_ascii_whitespace() {
                try_next!(self.stream);
            } else {
                break;
            }
        }

        // TODO: Consume comments

        // TODO: Collapse scanning routines

        if let Some(Ok(token)) = self.scan_numeric() {
            return Some(Ok(token));
        }

        if let Some(Ok(token)) = self.scan_identifier() {
            return Some(Ok(token));
        }

        // TODO: Error on unexpected character
        let ch = try_next!(self.stream);
        // TODO: Error log should be nicer to write
        eprintln!("{}: {}", "error".bold().red(), format!("unknown token: {}", ch).bold().white());

        None
    }

    fn scan_numeric(&mut self) -> Option<Result<Token, Error>> {
        if !try_peek!(self.stream).is_ascii_digit() { return None; }

        let mut value = String::new();
        // TODO: Keep track of stream position so we can know where this number begins

        while let Some(&Ok(ch)) = self.stream.peek() {
            if ch.is_ascii_digit() {
                value.push(ch);
                try_next!(self.stream);
            } else if ch == '_' && value.len() > 0 {
                try_next!(self.stream);
            } else {
                break;
            }
        }

        Some(Ok(Token::Integer { value, radix: 10 }))
    }

    fn scan_identifier(&mut self) -> Option<Result<Token, Error>> {
        let ch = try_peek!(self.stream);
        if !(ch.is_ascii_alphanumeric() || ch == '_') { return None; }

        let mut text = String::new();
        // TODO: Keep track of stream position so we can know where this number begins

        while let Some(&Ok(ch)) = self.stream.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                text.push(ch);
                try_next!(self.stream);
            } else {
                break;
            }
        }

        Some(Ok(Token::Identifier { text }))
    }
}

fn main() {
    let mut tokenizer = Tokenizer::new("./scratch.bolt").unwrap();

    while let Some(token) = tokenizer.next() {
        println!("{:?}", token.unwrap());
    }
}
