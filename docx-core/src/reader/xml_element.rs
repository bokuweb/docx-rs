use std::io::Read;
use std::str::FromStr;

use xml::reader::EventReader;

use crate::reader::ReaderError;

#[derive(PartialEq, Debug)]
pub enum XMLElement {
    Body,
    Paragraph,
    Run,
    RunProperty,
    Color,
    Underline,
    Size,
    SizeCs,
    Vanish,
    Italic,
    Text,
    Style,
    Highlight,
    Bold,
    BoldCS,
    Break,
    Tab,
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
            "color" => Ok(XMLElement::Color),
            "t" => Ok(XMLElement::Text),
            "sz" => Ok(XMLElement::Size),
            "szCs" => Ok(XMLElement::SizeCs),
            "u" => Ok(XMLElement::Underline),
            "style" => Ok(XMLElement::Style),
            "highlight" => Ok(XMLElement::Highlight),
            "b" => Ok(XMLElement::Bold),
            "bCs" => Ok(XMLElement::BoldCS),
            "vanish" => Ok(XMLElement::Vanish),
            "italic" => Ok(XMLElement::Italic),
            "tab" => Ok(XMLElement::Tab),
            "br" => Ok(XMLElement::Break),
            _ => Ok(XMLElement::Unsupported),
        }
    }
}

pub trait ElementReader {
    fn read<R: Read>(r: &mut EventReader<R>) -> Result<Self, ReaderError>
    where
        Self: std::marker::Sized;
}
