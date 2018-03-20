use std::fmt::{self, Display};
use std::collections::VecDeque;
use std::iter::Peekable;
use std::io::{Chars, Read};
use std::fs::File;
use failure::Error;
use location::Location;
use token::{Operator, Token, TokenKind};

macro_rules! try_opt {
    ($e:expr) => (
        match $e {
            Some(v) => v,
            None => return Ok(None)
        }
    )
}

#[derive(Debug, Fail)]
pub enum TokenizerError {
    UnknownToken(Location, char),
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenizerError::UnknownToken(ref location, ch) => {
                write!(f, "{}: ", location)?;
                write!(f, "unknown token: `{}`", ch)?;
            }
        }

        Ok(())
    }
}

pub struct Tokenizer {
    stream: Peekable<Chars<File>>,
    queue: VecDeque<Token>,

    /// Current location (line, column) in the stream.
    location: Location,
}

impl Tokenizer {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let stream = File::open(filename)?.chars().peekable();

        Ok(Self {
            stream,
            queue: VecDeque::new(),
            location: Location::new(filename, 1, 1),
        })
    }

    pub fn peek(&mut self) -> Result<Token, Error> {
        if self.queue.is_empty() {
            self.advance()?;
        }

        if self.queue.is_empty() {
            Ok(Token::new(self.location.clone(), TokenKind::EndOfFile))
        } else {
            Ok(self.queue[0].clone())
        }
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        if self.queue.is_empty() {
            self.advance()?;
        }

        if let Some(token) = self.queue.remove(0) {
            Ok(token)
        } else {
            Ok(Token::new(self.location.clone(), TokenKind::EndOfFile))
        }
    }

    fn advance(&mut self) -> Result<(), Error> {
        let token = {
            self.consume_whitespace()?;
            // TODO: Consume comments

            let location = self.location.clone();

            // TODO: Collapse scanning routines
            if let Some(kind) = self.scan_numeric()? {
                Token::new(location, kind)
            } else if let Some(kind) = self.scan_identifier()? {
                Token::new(location, kind)
            } else if let Some(kind) = self.scan_operator()? {
                Token::new(location, kind)
            } else if let Some(ch) = self.next_char()? {
                return Err(TokenizerError::UnknownToken(location, ch).into());
            } else {
                return Ok(());
            }
        };

        self.queue.push_back(token);

        Ok(())
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

    fn scan_numeric(&mut self) -> Result<Option<TokenKind>, Error> {
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

        Ok(Some(TokenKind::Integer { value, radix: 10 }))
    }

    fn scan_identifier(&mut self) -> Result<Option<TokenKind>, Error> {
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

        Ok(Some(TokenKind::Identifier { text }))
    }

    fn scan_operator(&mut self) -> Result<Option<TokenKind>, Error> {
        let ch = try_opt!(self.peek_char()?);
        let op = match ch {
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            '*' => Operator::Star,
            '/' => Operator::Slash,

            _ => return Ok(None),
        };

        try_opt!(self.next_char()?);

        Ok(Some(TokenKind::Operator(op)))
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
