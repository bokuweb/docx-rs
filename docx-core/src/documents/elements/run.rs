use super::{RunProperty, Text};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct Run {
    run_property: RunProperty,
    text: Text,
}

impl Run {
    pub fn new(text: impl Into<String>) -> Run {
        Run {
            text: Text::new(text),
            ..Default::default()
        }
    }
}

impl Default for Run {
    fn default() -> Self {
        let run_property = RunProperty::new();
        let text = Text::new("");
        Self { run_property, text }
    }
}

impl BuildXML for Run {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_run()
            .add_child(&self.run_property)
            .add_child(&self.text)
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
        let b = Run::new("Hello").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r><w:rPr /><w:t>Hello</w:t></w:r>"#
        );
    }
}
