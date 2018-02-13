/// A source code location used for error reporting
#[derive(Clone, Debug)]
pub struct Location {
    pub filename: String,

    /// The (1-based) line offset
    pub line: usize,

    /// The (0-based) character offset
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
