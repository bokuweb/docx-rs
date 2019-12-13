use crate::documents::BuildXML;
use crate::escape::escape;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Text {
    text: String,
    preserve_space: bool,
}

impl Text {
    pub fn new(text: &str) -> Text {
        Text {
            text: escape(text),
            preserve_space: true,
        }
    }
}

impl BuildXML for Text {
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
