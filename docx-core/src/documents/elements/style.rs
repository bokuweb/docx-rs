use crate::documents::BuildXML;
use crate::xml_builder::*;
use crate::StyleType;

use super::{BasedOn, Name, Next, ParagraphProperty, QFormat, RunProperty};

#[derive(Debug)]
pub struct Style {
    style_id: String,
    name: Name,
    style_type: StyleType,
    run_property: RunProperty,
    paragraph_property: ParagraphProperty,
}

impl Default for Style {
    fn default() -> Style {
        let name = Name::new("");
        let rpr = RunProperty::new();
        let ppr = ParagraphProperty::new();
        Style {
            style_id: "".to_owned(),
            style_type: StyleType::Paragraph,
            name,
            run_property: rpr,
            paragraph_property: ppr,
        }
    }
}

impl Style {
    pub fn new(
        style_id: impl Into<String>,
        name: impl Into<String>,
        style_type: StyleType,
    ) -> Style {
        let name = Name::new(name.into());
        let default = Default::default();
        Style {
            style_id: style_id.into(),
            style_type,
            name,
            ..default
        }
    }
}

impl BuildXML for Style {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        // Set "Normal" as default if you need change these values please fix it
        b.open_style(self.style_type, &self.style_id)
            .add_child(&self.name)
            .add_child(&self.run_property)
            .add_child(&self.paragraph_property)
            .add_child(&BasedOn::new("Normal"))
            .add_child(&Next::new("Normal"))
            .add_child(&QFormat::new())
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
        let c = Style::new("Heading", "Heading1", StyleType::Paragraph);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:style w:type="paragraph" w:styleId="Heading"><w:name w:val="Heading1" /><w:rPr /><w:pPr /><w:basedOn w:val="Normal" /><w:next w:val="Normal" /><w:qFormat /></w:style>"#
        );
    }
}
