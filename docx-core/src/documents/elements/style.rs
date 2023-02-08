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
    pub next: Option<Next>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
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
            next: None,
            link: None,
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

    pub fn next(mut self, next: impl Into<String>) -> Self {
        self.next = Some(Next::new(next));
        self
    }

    pub fn link(mut self, link: impl Into<String>) -> Self {
        self.link = Some(Link::new(link));
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

    pub fn text_border(mut self, b: TextBorder) -> Self {
        self.run_property = self.run_property.text_border(b);
        self
    }

    pub fn fonts(mut self, f: RunFonts) -> Self {
        self.run_property = self.run_property.fonts(f);
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

    pub fn table_indent(mut self, v: i32) -> Self {
        self.table_property = self.table_property.indent(v);
        self
    }

    pub fn table_align(mut self, v: TableAlignmentType) -> Self {
        self.table_property = self.table_property.align(v);
        self
    }

    pub fn style(mut self, s: impl Into<String>) -> Self {
        self.table_property = self.table_property.style(s);
        self
    }

    pub fn layout(mut self, t: TableLayoutType) -> Self {
        self.table_property = self.table_property.layout(t);
        self
    }

    pub fn width(mut self, w: usize, t: WidthType) -> Self {
        self.table_property = self.table_property.width(w, t);
        self
    }

    pub fn margins(mut self, margins: TableCellMargins) -> Self {
        self.table_property = self.table_property.set_margins(margins);
        self
    }

    pub fn set_borders(mut self, borders: TableBorders) -> Self {
        self.table_property = self.table_property.set_borders(borders);
        self
    }

    pub fn set_border(mut self, border: TableBorder) -> Self {
        self.table_property = self.table_property.set_border(border);
        self
    }

    pub fn clear_border(mut self, position: TableBorderPosition) -> Self {
        self.table_property = self.table_property.clear_border(position);
        self
    }

    pub fn clear_all_border(mut self) -> Self {
        self.table_property = self.table_property.clear_all_border();
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
        let mut b = b
            .open_style(self.style_type, &self.style_id)
            .add_child(&self.name)
            .add_child(&self.run_property)
            .add_child(&self.paragraph_property);

        if self.style_type == StyleType::Table {
            b = b
                .add_child(&self.table_cell_property)
                .add_child(&self.table_property);
        }

        if let Some(ref next) = self.next {
            b = b.add_child(next)
        }

        if let Some(ref link) = self.link {
            b = b.add_child(link)
        }

        b.add_child(&QFormat::new())
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
            r#"<w:style w:type="paragraph" w:styleId="Heading"><w:name w:val="Heading1" /><w:rPr /><w:pPr><w:rPr /></w:pPr><w:qFormat /></w:style>"#
        );
    }
}
