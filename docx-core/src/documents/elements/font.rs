use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;
use std::io::Write;

#[derive(Debug)]
pub struct Font<'a> {
    name: &'a str,
    charset: &'a str,
    family: &'a str,
    pitch: FontPitchType,
}

impl<'a> Font<'a> {
    pub fn new(name: &'a str, charset: &'a str, family: &'a str, pitch: FontPitchType) -> Font<'a> {
        Font {
            name,
            charset,
            family,
            pitch,
        }
    }
}

impl<'a> BuildXML for Font<'a> {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_font(self.name)?
            .charset(self.charset)?
            .family(self.family)?
            .pitch(&self.pitch.to_string())?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_build() {
        let c = Font::new("Arial", "00", "swiss", FontPitchType::Variable);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:font w:name="Arial"><w:charset w:val="00" /><w:family w:val="swiss" /><w:pitch w:val="variable" /></w:font>"#
        );
    }
}
