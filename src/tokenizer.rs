use std::iter::Peekable;
use std::io::{Chars, Read};
use std::fs::File;
use failure::Error;
use location::Location;
use token::Token;

macro_rules! try_opt {
    ($e:expr) => (
        match $e {
            Some(v) => v,
            None => return Ok(None)
        }
    )
}

pub struct Tokenizer {
    stream: Peekable<Chars<File>>,

    /// Current location (line, column) in the stream.
    location: Location,
}

impl Tokenizer {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let stream = File::open(filename)?.chars().peekable();

        Ok(Self {
            stream,
            location: Location::new(filename, 1, 1),
        })
    }

    pub fn next(&mut self) -> Result<Option<(Location, Token)>, Error> {
        self.consume_whitespace()?;

        let location = self.location.clone();

        // TODO: Consume comments
        // TODO: Collapse scanning routines

        if let Some(token) = self.scan_numeric()? {
            return Ok(Some((location, token)));
        }

        if let Some(token) = self.scan_identifier()? {
            return Ok(Some((location, token)));
        }

        // TODO: Error on unexpected character
        if let Some(ch) = self.next_char()? {
            // TODO: Use log
            use colored::Colorize;

            eprintln!(
                "{} {} {}",
                format!("{}:", location).to_string().bold().white(),
                "error:".bold().red(),
                format!("unknown token: {}", ch).bold().white()
            );
        }

        Ok(None)
    }

    fn consume_whitespace(&mut self) -> Result<(), Error> {
        while let Some(ch) = self.peek_char()? {
            if ch.is_ascii_whitespace() {
                self.next_char()?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn scan_numeric(&mut self) -> Result<Option<Token>, Error> {
        if !try_opt!(self.peek_char()?).is_ascii_digit() {
            return Ok(None);
        }

        let mut value = String::new();
        // TODO: Keep track of stream position so we can know where this number begins

        while let Some(&Ok(ch)) = self.stream.peek() {
            if ch.is_ascii_digit() {
                value.push(ch);
                try_opt!(self.next_char()?);
            } else if ch == '_' && value.len() > 0 {
                try_opt!(self.next_char()?);
            } else {
                break;
            }
        }

        Ok(Some(Token::Integer { value, radix: 10 }))
    }

    fn scan_identifier(&mut self) -> Result<Option<Token>, Error> {
        let ch = try_opt!(self.peek_char()?);
        if !(ch.is_ascii_alphanumeric() || ch == '_') {
            return Ok(None);
        }

        let mut text = String::new();
        // TODO: Keep track of stream position so we can know where this number begins

        while let Some(&Ok(ch)) = self.stream.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                text.push(ch);
                try_opt!(self.next_char()?);
            } else {
                break;
            }
        }

        Ok(Some(Token::Identifier { text }))
    }

    fn next_char(&mut self) -> Result<Option<char>, Error> {
        match self.stream.next() {
            Some(Ok(ch)) => {
                if ch == '\n' {
                    self.location.line += 1;
                    self.location.column = 1;
                } else {
                    self.location.column += 1;
                }

                Ok(Some(ch))
            }
            Some(Err(error)) => Err(error.into()),
            None => Ok(None),
        }
    }

    fn peek_char(&mut self) -> Result<Option<char>, Error> {
        match self.stream.peek() {
            Some(&Ok(ch)) => Ok(Some(ch)),
            _ => Ok(None),
        }
    }
}
