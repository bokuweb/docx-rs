use super::{RunProperty, Sz};
use crate::documents::BuildXML;
use crate::xml_builder::*;

pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Text {
        Text { text: text.into() }
    }
}

impl BuildXML for Text {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.text(&self.text).build()
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
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:t>Hello</w:t>"#);
    }
}
