use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;
use crate::StyleType;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub style_id: String,
    pub name: Name,
    pub style_type: StyleType,
    pub run_property: RunProperty,
    pub paragraph_property: ParagraphProperty,
    pub table_property: TableProperty,
    pub table_cell_property: TableCellProperty,
    pub based_on: Option<BasedOn>,
}

impl Default for Style {
    fn default() -> Self {
        let name = Name::new("");
        let rpr = RunProperty::new();
        let ppr = ParagraphProperty::new();
        Style {
            style_id: "".to_owned(),
            style_type: StyleType::Paragraph,
            name,
            run_property: rpr,
            paragraph_property: ppr,
            table_property: TableProperty::new(),
            table_cell_property: TableCellProperty::new(),
            based_on: None,
        }
    }
}

impl Style {
    pub fn new(style_id: impl Into<String>, style_type: StyleType) -> Self {
        let default = Default::default();
        Style {
            style_id: style_id.into(),
            style_type,
            ..default
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Name::new(name);
        self
    }

    pub fn based_on(mut self, base: impl Into<String>) -> Self {
        self.based_on = Some(BasedOn::new(base));
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.run_property = self.run_property.size(size);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.run_property = self.run_property.color(color);
        self
    }

    pub fn highlight(mut self, color: impl Into<String>) -> Self {
        self.run_property = self.run_property.highlight(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.run_property = self.run_property.bold();
        self
    }

    pub fn italic(mut self) -> Self {
        self.run_property = self.run_property.italic();
        self
    }

    pub fn underline(mut self, line_type: impl Into<String>) -> Self {
        self.run_property = self.run_property.underline(line_type);
        self
    }

    pub fn vanish(mut self) -> Self {
        self.run_property = self.run_property.vanish();
        self
    }

    pub fn align(mut self, alignment_type: AlignmentType) -> Self {
        self.paragraph_property = self.paragraph_property.align(alignment_type);
        self
    }

    pub fn indent(
        mut self,
        left: Option<i32>,
        special_indent: Option<SpecialIndentType>,
        end: Option<i32>,
        start_chars: Option<i32>,
    ) -> Self {
        self.paragraph_property =
            self.paragraph_property
                .indent(left, special_indent, end, start_chars);
        self
    }

    pub fn hanging_chars(mut self, chars: i32) -> Self {
        self.paragraph_property = self.paragraph_property.hanging_chars(chars);
        self
    }

    pub fn first_line_chars(mut self, chars: i32) -> Self {
        self.paragraph_property = self.paragraph_property.first_line_chars(chars);
        self
    }

    pub fn outline_lvl(mut self, l: usize) -> Self {
        self.paragraph_property = self.paragraph_property.outline_lvl(l);
        self
    }

    pub fn table_property(mut self, p: TableProperty) -> Self {
        self.table_property = p;
        self
    }

    pub fn table_cell_property(mut self, p: TableCellProperty) -> Self {
        self.table_cell_property = p;
        self
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
            .add_optional_child(&self.based_on)
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
        let c = Style::new("Heading", StyleType::Paragraph).name("Heading1");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:style w:type="paragraph" w:styleId="Heading"><w:name w:val="Heading1" /><w:rPr /><w:pPr><w:rPr /></w:pPr><w:basedOn w:val="Normal" /><w:next w:val="Normal" /><w:qFormat /></w:style>"#
        );
    }
}
