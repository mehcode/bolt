use location::Location;

#[derive(Debug, Clone)]
pub struct Token {
    pub location: Location,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(location: Location, kind: TokenKind) -> Self {
        Self { location, kind }
    }
}

// TODO: Symbol can evolve into an interned string type
pub type Symbol = String;

#[derive(Clone, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Slash,
    Star,
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    EndOfFile,
    Identifier { text: Symbol },
    Integer { value: Symbol, radix: u8 },
    Operator(Operator),
}

impl TokenKind {
    pub fn is_binary_operator(&self) -> bool {
        match *self {
            TokenKind::Operator(Operator::Plus)
            | TokenKind::Operator(Operator::Minus)
            | TokenKind::Operator(Operator::Slash)
            | TokenKind::Operator(Operator::Star) => true,

            _ => false,
        }
    }
}
