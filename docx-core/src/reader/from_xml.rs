use crate::reader::ReaderError;
use std::io::Read;

pub trait FromXML {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError>
    where
        Self: std::marker::Sized;
}
