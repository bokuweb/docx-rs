use crate::xml_builder::XMLBuilder;
use std::io::Write;

pub trait BuildXML {
    /// Write XML to the output stream.
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>>;

    fn build(&self) -> Vec<u8> {
        self.build_to(XMLBuilder::new(Vec::new()).into_inner().unwrap())
            .expect("should write to buf")
            .into_inner()
    }
}

impl<'a, T: BuildXML> BuildXML for &'a T {
    /// Building XML from `&T` is the same as from `T`.
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        (*self).build_to(stream)
    }
}

impl<T: BuildXML> BuildXML for Box<T> {
    /// Building XML from `Box<T>` is the same as from `T`.
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        (**self).build_to(stream)
    }
}
