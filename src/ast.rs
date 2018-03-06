use location::Location;
use symbol::Symbol;
use std::io::Write;
use xml::writer::{EventWriter, Result as XmlResult, XmlEvent};

pub trait Serialize {
    fn serialize<W: Write>(&self, w: &mut EventWriter<W>) -> XmlResult<()>;
}

/// A literal (for example: `231`)
#[derive(Debug, Clone)]
pub struct Literal {
    pub node: LiteralKind,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Integer(Symbol),
}

impl Serialize for Literal {
    fn serialize<W: Write>(&self, w: &mut EventWriter<W>) -> XmlResult<()> {
        match self.node {
            LiteralKind::Integer(ref value) => {
                w.write(
                    XmlEvent::start_element("Integer").attr("location", &self.location.to_string()),
                )?;

                w.write(XmlEvent::characters(value))?;
            }
        }

        w.write(XmlEvent::end_element())?;

        Ok(())
    }
}

/// A binary operation (for example: `a + b` or `a * b`)
#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// An expression
#[derive(Debug, Clone)]
pub struct Expression {
    pub node: ExpressionKind,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Literal(Box<Literal>),
    Binary(BinaryOperation, Box<Expression>, Box<Expression>),
}

impl Serialize for Expression {
    fn serialize<W: Write>(&self, w: &mut EventWriter<W>) -> XmlResult<()> {
        match self.node {
            ExpressionKind::Literal(ref lit) => {
                lit.serialize(w)?;
            }

            ExpressionKind::Binary(ref operation, ref a, ref b) => {
                let op_name = match operation {
                    BinaryOperation::Add => "Add",
                    BinaryOperation::Subtract => "Subtract",
                    BinaryOperation::Multiply => "Multiply",
                    BinaryOperation::Divide => "Divide",
                };

                w.write(
                    XmlEvent::start_element(op_name).attr("location", &self.location.to_string()),
                )?;

                a.serialize(w)?;
                b.serialize(w)?;

                w.write(XmlEvent::end_element())?;
            }
        }

        Ok(())
    }
}
