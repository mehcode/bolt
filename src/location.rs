use std::fmt::{self, Display};

/// A source code location used for error reporting
#[derive(Clone, Debug)]
pub struct Location {
    /// The (1-based) line offset
    pub line: usize,

    /// The (1-based) character offset
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
