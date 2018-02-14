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
pub enum Token {
    Identifier { text: Symbol },
    Integer { value: Symbol, radix: u8 },
    Operator(Operator),
}

impl Token {
    pub fn is_binary_operator(&self) -> bool {
        match *self {
            Token::Operator(Operator::Plus) |
            Token::Operator(Operator::Minus) |
            Token::Operator(Operator::Slash) |
            Token::Operator(Operator::Star) => true,

            _ => false
        }
    }
}
