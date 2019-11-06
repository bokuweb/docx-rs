use crate::documents::BuildXML;
use crate::xml_builder::*;

use super::{BasedOn, Name, Next, ParagraphProperty, QFormat, RunProperty};

pub struct Style {
    style_id: String,
    name: Name,
    style_type: StyleType,
    run_property: RunProperty,
    paragraph_property: ParagraphProperty,
}

impl Style {
    pub fn new(
        style_id: impl Into<String>,
        name: impl Into<String>,
        style_type: StyleType,
    ) -> Style {
        let name = Name::new(name.into());
        let rpr = RunProperty::new();
        let ppr = ParagraphProperty::new();
        Style {
            style_id: style_id.into(),
            style_type,
            name,
            run_property: rpr,
            paragraph_property: ppr,
        }
    }
}

impl BuildXML for Style {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let name = self.name.build();
        let rpr = self.run_property.build();
        let ppr = self.paragraph_property.build();
        let based_on = BasedOn::new("Normal").build();
        let next = Next::new("Normal").build();
        let q_format = QFormat::new().build();
        b.open_style(self.style_type, &self.style_id)
            .add_child_buffer(&name)
            .add_child_buffer(&rpr)
            .add_child_buffer(&ppr)
            .add_child_buffer(&based_on)
            .add_child_buffer(&next)
            .add_child_buffer(&q_format)
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
