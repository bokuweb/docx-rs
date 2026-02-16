use crate::xml_builder::XMLBuilder;
use std::io::Write;

pub trait BuildXML {
    /// Write XML to the output stream.
    #[doc(hidden)]
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>>;

    #[doc(hidden)]
    fn build(&self) -> Vec<u8> {
        self.build_to(
            XMLBuilder::new(Vec::new())
                .into_inner()
                .expect("failed to create XML builder"),
        )
        .expect("should write to buf")
        .into_inner()
        .expect("failed to extract buffer")
    }
}

impl<'a, T: BuildXML> BuildXML for &'a T {
    /// Building XML from `&T` is the same as from `T`.
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        (*self).build_to(stream)
    }
}

impl<T: BuildXML> BuildXML for Box<T> {
    /// Building XML from `Box<T>` is the same as from `T`.
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        (**self).build_to(stream)
    }
}
