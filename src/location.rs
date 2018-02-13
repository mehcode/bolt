use std::fmt::{self, Display};

/// A source code location used for error reporting
#[derive(Clone, Debug)]
pub struct Location {
    pub filename: String,

    /// The (1-based) line offset
    pub line: usize,

    /// The (1-based) character offset
    pub column: usize,
}

impl Location {
    pub fn new<S: Into<String>>(filename: S, line: usize, column: usize) -> Self {
        Self {
            filename: filename.into(),
            line,
            column,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}
