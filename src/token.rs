// TODO: Symbol can evolve into an interned string type
pub type Symbol = String;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Slash,
    Star,
    Percent,
}

#[derive(Debug)]
pub enum Token {
    Identifier { text: Symbol },
    Integer { value: Symbol, radix: u8 },
    Operator(Operator),
}
