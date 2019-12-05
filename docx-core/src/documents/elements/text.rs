use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Text<'a> {
    text: &'a str,
    preserve_space: bool,
}

impl<'a> Text<'a> {
    pub fn new(text: &'a str) -> Text {
        Text {
            text: text.into(),
            preserve_space: true,
        }
    }
}

impl<'a> BuildXML for Text<'a> {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().text(&self.text, true).build()
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
        let b = Text::new("Hello").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:t xml:space="preserve">Hello</w:t>"#
        );
    }
}
