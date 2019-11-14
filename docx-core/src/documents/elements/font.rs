use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

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
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_font(self.name)
            .charset(self.charset)
            .family(self.family)
            .pitch(&self.pitch.to_string())
            .close()
            .build()
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
            r#"<w:font w:name="Arial">
  <w:charset w:val="00" />
  <w:family w:val="swiss" />
  <w:pitch w:val="variable" />
</w:font>"#
        );
    }
}
