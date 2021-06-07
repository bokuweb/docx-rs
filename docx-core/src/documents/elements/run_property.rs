use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RunProperty {
    pub sz: Option<Sz>,
    pub sz_cs: Option<SzCs>,
    pub color: Option<Color>,
    pub highlight: Option<Highlight>,
    pub vert_align: Option<VertAlign>,
    pub underline: Option<Underline>,
    pub bold: Option<Bold>,
    pub bold_cs: Option<BoldCs>,
    pub italic: Option<Italic>,
    pub italic_cs: Option<ItalicCs>,
    pub vanish: Option<Vanish>,
    pub spacing: Option<i32>,
    pub fonts: Option<RunFonts>,
    pub text_border: Option<TextBorder>,
    pub del: Option<Delete>,
    pub ins: Option<Insert>,
}

impl RunProperty {
    pub fn new() -> RunProperty {
        Default::default()
    }

    pub fn size(mut self, size: usize) -> RunProperty {
        self.sz = Some(Sz::new(size));
        self.sz_cs = Some(SzCs::new(size));
        self
    }

    pub fn spacing(mut self, spacing: i32) -> RunProperty {
        self.spacing = Some(spacing);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> RunProperty {
        self.color = Some(Color::new(color));
        self
    }

    pub fn highlight(mut self, color: impl Into<String>) -> RunProperty {
        self.highlight = Some(Highlight::new(color));
        self
    }

    pub fn vert_align(mut self, a: VertAlignType) -> Self {
        self.vert_align = Some(VertAlign::new(a));
        self
    }

    pub fn bold(mut self) -> RunProperty {
        self.bold = Some(Bold::new());
        self.bold_cs = Some(BoldCs::new());
        self
    }

    pub fn disable_bold(mut self) -> RunProperty {
        self.bold = Some(Bold::new().disable());
        self.bold_cs = Some(BoldCs::new().disable());
        self
    }

    pub fn italic(mut self) -> RunProperty {
        self.italic = Some(Italic::new());
        self.italic_cs = Some(ItalicCs::new());
        self
    }

    pub fn disable_italic(mut self) -> RunProperty {
        self.italic = Some(Italic::new().disable());
        self.italic_cs = Some(ItalicCs::new().disable());
        self
    }

    pub fn underline(mut self, line_type: impl Into<String>) -> RunProperty {
        self.underline = Some(Underline::new(line_type));
        self
    }

    pub fn vanish(mut self) -> RunProperty {
        self.vanish = Some(Vanish::new());
        self
    }

    pub fn fonts(mut self, font: RunFonts) -> RunProperty {
        self.fonts = Some(font);
        self
    }

    pub fn text_border(mut self, b: TextBorder) -> Self {
        self.text_border = Some(b);
        self
    }

    pub fn delete(mut self, d: Delete) -> Self {
        self.del = Some(d);
        self
    }

    pub fn insert(mut self, i: Insert) -> Self {
        self.ins = Some(i);
        self
    }
}

impl Default for RunProperty {
    fn default() -> Self {
        Self {
            color: None,
            sz: None,
            sz_cs: None,
            highlight: None,
            vert_align: None,
            underline: None,
            bold: None,
            bold_cs: None,
            italic: None,
            italic_cs: None,
            vanish: None,
            fonts: None,
            spacing: None,
            text_border: None,
            del: None,
            ins: None,
        }
    }
}

impl BuildXML for RunProperty {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let spacing = self
            .spacing
            .map(|s| Spacing::new(crate::SpacingType::Value(s)));
        b.open_run_property()
            .add_optional_child(&self.sz)
            .add_optional_child(&self.sz_cs)
            .add_optional_child(&self.color)
            .add_optional_child(&self.bold)
            .add_optional_child(&self.bold_cs)
            .add_optional_child(&self.italic)
            .add_optional_child(&self.italic_cs)
            .add_optional_child(&self.highlight)
            .add_optional_child(&self.underline)
            .add_optional_child(&self.vanish)
            .add_optional_child(&self.fonts)
            .add_optional_child(&self.text_border)
            .add_optional_child(&self.ins)
            .add_optional_child(&self.del)
            .add_optional_child(&self.vert_align)
            .add_optional_child(&spacing)
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
    fn test_size() {
        let c = RunProperty::new().size(10).color("FFFFFF");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:sz w:val="10" /><w:szCs w:val="10" /><w:color w:val="FFFFFF" /></w:rPr>"#
        );
    }

    #[test]
    fn test_highlight() {
        let c = RunProperty::new().highlight("FFFFFF");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:highlight w:val="FFFFFF" /></w:rPr>"#
        );
    }

    #[test]
    fn test_bold() {
        let c = RunProperty::new().bold();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:b /><w:bCs /></w:rPr>"#
        );
    }

    #[test]
    fn test_underline() {
        let c = RunProperty::new().underline("single");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:u w:val="single" /></w:rPr>"#
        );
    }

    #[test]
    fn test_vanish() {
        let c = RunProperty::new().vanish();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:vanish /></w:rPr>"#
        );
    }

    #[test]
    fn test_run_fonts() {
        let c = RunProperty::new().fonts(RunFonts::new().east_asia("Hiragino"));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:rFonts w:eastAsia="Hiragino" /></w:rPr>"#
        );
    }
}
