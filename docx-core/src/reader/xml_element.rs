use std::io::Read;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};

use crate::reader::ReaderError;

#[derive(PartialEq, Debug)]
pub enum XMLElement {
    Body,
    Paragraph,
    Run,
    RunProperty,
    Unsupported,
}

impl FromStr for XMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "body" => Ok(XMLElement::Body),
            "p" => Ok(XMLElement::Paragraph),
            "r" => Ok(XMLElement::Run),
            "rPr" => Ok(XMLElement::RunProperty),
            _ => Ok(XMLElement::Unsupported),
        }
    }
}

pub trait ElementReader {
    fn read<R: Read>(r: &mut EventReader<R>) -> Result<Self, ReaderError>
    where
        Self: std::marker::Sized;
}
