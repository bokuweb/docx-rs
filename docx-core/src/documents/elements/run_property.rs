use serde::{Deserialize, Serialize};

use super::{Bold, BoldCs, Color, Highlight, Italic, ItalicCs, Sz, SzCs, Underline, Vanish};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RunProperty {
    pub sz: Option<Sz>,
    pub sz_cs: Option<SzCs>,
    pub color: Option<Color>,
    pub highlight: Option<Highlight>,
    pub underline: Option<Underline>,
    pub bold: Option<Bold>,
    pub bold_cs: Option<BoldCs>,
    pub italic: Option<Italic>,
    pub italic_cs: Option<ItalicCs>,
    pub vanish: Option<Vanish>,
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

    pub fn color(mut self, color: impl Into<String>) -> RunProperty {
        self.color = Some(Color::new(color));
        self
    }

    pub fn highlight(mut self, color: impl Into<String>) -> RunProperty {
        self.highlight = Some(Highlight::new(color));
        self
    }

    pub fn bold(mut self) -> RunProperty {
        self.bold = Some(Bold::new());
        self.bold_cs = Some(BoldCs::new());
        self
    }

    pub fn italic(mut self) -> RunProperty {
        self.italic = Some(Italic::new());
        self.italic_cs = Some(ItalicCs::new());
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
}

impl Default for RunProperty {
    fn default() -> Self {
        Self {
            color: None,
            sz: None,
            sz_cs: None,
            highlight: None,
            underline: None,
            bold: None,
            bold_cs: None,
            italic: None,
            italic_cs: None,
            vanish: None,
        }
    }
}

impl BuildXML for RunProperty {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
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
}
