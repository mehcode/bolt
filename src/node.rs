use token::Symbol;
use std::io::Write;
use xml::writer::{EventWriter, Result as XmlResult, XmlEvent};

pub trait WriteXml {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) -> XmlResult<()>;
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer { value: Symbol },
}

impl WriteXml for Literal {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) -> XmlResult<()> {
        match *self {
            Literal::Integer { ref value } => {
                w.write(XmlEvent::start_element("Integer"))?;
                w.write(XmlEvent::characters(value))?;
                w.write(XmlEvent::end_element())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
}

impl WriteXml for BinaryOperation {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) -> XmlResult<()> {
        match *self {
            BinaryOperation::Add(..) => {
                w.write(XmlEvent::start_element("Add"))?;
            }

            BinaryOperation::Subtract(..) => {
                w.write(XmlEvent::start_element("Subtract"))?;
            }

            BinaryOperation::Multiply(..) => {
                w.write(XmlEvent::start_element("Multiply"))?;
            }

            BinaryOperation::Divide(..) => {
                w.write(XmlEvent::start_element("Divide"))?;
            }
        }

        match *self {
            BinaryOperation::Add(ref a, ref b)
            | BinaryOperation::Subtract(ref a, ref b)
            | BinaryOperation::Multiply(ref a, ref b)
            | BinaryOperation::Divide(ref a, ref b) => {
                a.write_xml(w)?;
                b.write_xml(w)?;
            }
        }

        w.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    BinaryOperation(BinaryOperation),
}

impl WriteXml for Expression {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) -> XmlResult<()> {
        match *self {
            Expression::Literal(ref lit) => lit.write_xml(w),
            Expression::BinaryOperation(ref op) => op.write_xml(w),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Expression(Expression),
}

impl WriteXml for Node {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) -> XmlResult<()> {
        match *self {
            Node::Expression(ref expr) => expr.write_xml(w),
        }
    }
}
