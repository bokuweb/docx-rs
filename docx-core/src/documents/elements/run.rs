use super::*;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub run_property: RunProperty,
    pub children: Vec<RunChild>,
}

impl Default for Run {
    fn default() -> Self {
        let run_property = RunProperty::new();
        Self {
            run_property,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RunChild {
    Text(Text),
    Sym(Sym),
    DeleteText(DeleteText),
    Tab(Tab),
    PTab(PositionalTab),
    Break(Break),
    Drawing(Box<Drawing>),
    Shape(Box<Shape>),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
    FieldChar(FieldChar),
    InstrText(Box<InstrText>),
    DeleteInstrText(Box<DeleteInstrText>),
    // For reader
    InstrTextString(String),
    FootnoteReference(FootnoteReference),
    Shading(Shading),
}

impl Serialize for RunChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RunChild::Text(ref s) => {
                let mut t = serializer.serialize_struct("Text", 2)?;
                t.serialize_field("type", "text")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Sym(ref s) => {
                let mut t = serializer.serialize_struct("Sym", 2)?;
                t.serialize_field("type", "sym")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::DeleteText(ref s) => {
                let mut t = serializer.serialize_struct("DeleteText", 2)?;
                t.serialize_field("type", "deleteText")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Tab(_) => {
                let mut t = serializer.serialize_struct("Tab", 1)?;
                t.serialize_field("type", "tab")?;
                t.end()
            }
            RunChild::PTab(ref s) => {
                let mut t = serializer.serialize_struct("PTab", 1)?;
                t.serialize_field("type", "ptab")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Break(ref s) => {
                let mut t = serializer.serialize_struct("Break", 2)?;
                t.serialize_field("type", "break")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Drawing(ref s) => {
                let mut t = serializer.serialize_struct("Drawing", 2)?;
                t.serialize_field("type", "drawing")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Shape(ref s) => {
                let mut t = serializer.serialize_struct("Shape", 2)?;
                t.serialize_field("type", "shape")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::CommentStart(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeStart", 2)?;
                t.serialize_field("type", "commentRangeStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            RunChild::CommentEnd(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeEnd", 2)?;
                t.serialize_field("type", "commentRangeEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            RunChild::FieldChar(ref f) => {
                let mut t = serializer.serialize_struct("FieldChar", 2)?;
                t.serialize_field("type", "fieldChar")?;
                t.serialize_field("data", f)?;
                t.end()
            }
            RunChild::InstrText(ref i) => {
                let mut t = serializer.serialize_struct("InstrText", 2)?;
                t.serialize_field("type", "instrText")?;
                t.serialize_field("data", i)?;
                t.end()
            }
            RunChild::DeleteInstrText(ref i) => {
                let mut t = serializer.serialize_struct("DeleteInstrText", 2)?;
                t.serialize_field("type", "deleteInstrText")?;
                t.serialize_field("data", i)?;
                t.end()
            }
            RunChild::InstrTextString(ref i) => {
                let mut t = serializer.serialize_struct("InstrTextString", 2)?;
                t.serialize_field("type", "instrTextString")?;
                t.serialize_field("data", i)?;
                t.end()
            }
            RunChild::FootnoteReference(ref f) => {
                let mut t = serializer.serialize_struct("FootnoteReference", 2)?;
                t.serialize_field("type", "footnoteReference")?;
                t.serialize_field("data", f)?;
                t.end()
            }
            RunChild::Shading(ref f) => {
                let mut t = serializer.serialize_struct("Shading", 2)?;
                t.serialize_field("type", "shading")?;
                t.serialize_field("data", f)?;
                t.end()
            }
        }
    }
}

impl Run {
    pub fn new() -> Run {
        Run {
            ..Default::default()
        }
    }

    pub fn add_text(mut self, text: impl Into<String>) -> Run {
        self.children
            .push(RunChild::Text(Text::new(text.into().replace('\n', ""))));
        self
    }

    pub(crate) fn add_text_without_escape(mut self, text: impl Into<String>) -> Run {
        self.children.push(RunChild::Text(Text::without_escape(
            text.into().replace('\n', ""),
        )));
        self
    }

    pub fn add_delete_text(mut self, text: impl Into<String>) -> Run {
        self.children.push(RunChild::DeleteText(DeleteText::new(
            text.into().replace('\n', ""),
        )));
        self
    }

    pub(crate) fn add_delete_text_without_escape(mut self, text: impl Into<String>) -> Run {
        self.children
            .push(RunChild::DeleteText(DeleteText::without_escape(
                text.into().replace('\n', ""),
            )));
        self
    }

    pub fn add_field_char(mut self, t: crate::types::FieldCharType, dirty: bool) -> Run {
        let mut f = FieldChar::new(t);
        if dirty {
            f = f.dirty();
        };
        self.children.push(RunChild::FieldChar(f));
        self
    }

    pub fn add_tc(mut self, tc: InstrTC) -> Run {
        self = self.add_field_char(crate::types::FieldCharType::Begin, false);
        self = self.add_instr_text(InstrText::TC(tc));
        self = self.add_field_char(crate::types::FieldCharType::End, false);
        self
    }

    pub fn add_instr_text(mut self, i: InstrText) -> Run {
        self.children.push(RunChild::InstrText(Box::new(i)));
        self
    }

    pub fn add_delete_instr_text(mut self, i: DeleteInstrText) -> Run {
        self.children.push(RunChild::DeleteInstrText(Box::new(i)));
        self
    }

    pub fn add_tab(mut self) -> Run {
        self.children.push(RunChild::Tab(Tab::new()));
        self
    }

    pub fn add_ptab(mut self, ptab: PositionalTab) -> Run {
        self.children.push(RunChild::PTab(ptab));
        self
    }

    pub fn add_image(mut self, pic: Pic) -> Run {
        self.children
            .push(RunChild::Drawing(Box::new(Drawing::new().pic(pic))));
        self
    }

    pub(crate) fn add_drawing(mut self, d: Drawing) -> Run {
        self.children.push(RunChild::Drawing(Box::new(d)));
        self
    }

    // For now reader only
    //    pub(crate) fn add_shape(mut self, d: Shape) -> Run {
    //        self.children.push(RunChild::Shape(Box::new(d)));
    //        self
    //    }

    pub fn add_break(mut self, break_type: BreakType) -> Run {
        self.children.push(RunChild::Break(Break::new(break_type)));
        self
    }

    pub fn add_sym(mut self, sym: Sym) -> Run {
        self.children.push(RunChild::Sym(sym));
        self
    }

    pub fn style(mut self, style_id: &str) -> Self {
        self.run_property = self.run_property.style(style_id);
        self
    }

    pub fn size(mut self, size: usize) -> Run {
        self.run_property = self.run_property.size(size);
        self
    }

    pub fn character_spacing(mut self, v: i32) -> Run {
        self.run_property = self.run_property.spacing(v);
        self
    }

    pub fn stretch(mut self, v: i32) -> Run {
        self.run_property = self.run_property.stretch(v);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Run {
        self.run_property = self.run_property.color(color);
        self
    }

    pub fn highlight(mut self, color: impl Into<String>) -> Run {
        self.run_property = self.run_property.highlight(color);
        self
    }

    pub fn bold(mut self) -> Run {
        self.run_property = self.run_property.bold();
        self
    }

    pub fn disable_bold(mut self) -> Run {
        self.run_property = self.run_property.disable_bold();
        self
    }

    pub fn italic(mut self) -> Run {
        self.run_property = self.run_property.italic();
        self
    }

    pub fn strike(mut self) -> Run {
        self.run_property = self.run_property.strike();
        self
    }

    pub fn dstrike(mut self) -> Run {
        self.run_property = self.run_property.dstrike();
        self
    }

    pub fn text_border(mut self, b: TextBorder) -> Run {
        self.run_property = self.run_property.text_border(b);
        self
    }

    pub fn disable_italic(mut self) -> Run {
        self.run_property = self.run_property.disable_italic();
        self
    }

    pub fn underline(mut self, line_type: impl Into<String>) -> Run {
        self.run_property = self.run_property.underline(line_type);
        self
    }

    pub fn vanish(mut self) -> Run {
        self.run_property = self.run_property.vanish();
        self
    }

    pub fn fonts(mut self, f: RunFonts) -> Run {
        self.run_property = self.run_property.fonts(f);
        self
    }

    pub(crate) fn set_property(mut self, p: RunProperty) -> Run {
        self.run_property = p;
        self
    }

    pub fn add_footnote_reference(mut self, footnote: Footnote) -> Run {
        self.run_property = RunProperty::new().style("FootnoteReference");
        self.children
            .push(RunChild::FootnoteReference(footnote.into()));
        self
    }

    pub fn shading(mut self, shading: Shading) -> Run {
        self.run_property = self.run_property.shading(shading);
        self
    }
}

impl BuildXML for RunChild {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        match self {
            RunChild::Text(t) => t.build_to(stream),
            RunChild::Sym(t) => t.build_to(stream),
            RunChild::DeleteText(t) => t.build_to(stream),
            RunChild::Tab(t) => t.build_to(stream),
            RunChild::PTab(t) => t.build_to(stream),
            RunChild::Break(t) => t.build_to(stream),
            RunChild::Drawing(t) => t.build_to(stream),
            RunChild::Shape(_t) => {
                todo!("Support shape writer.")
            }
            RunChild::CommentStart(c) => c.build_to(stream),
            RunChild::CommentEnd(c) => c.build_to(stream),
            RunChild::FieldChar(c) => c.build_to(stream),
            RunChild::InstrText(c) => c.build_to(stream),
            RunChild::DeleteInstrText(c) => c.build_to(stream),
            RunChild::InstrTextString(_) => unreachable!(),
            RunChild::FootnoteReference(c) => c.build_to(stream),
            RunChild::Shading(s) => s.build_to(stream),
        }
    }
}

impl BuildXML for Run {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_run()?
            .add_child(&self.run_property)?
            .add_children(&self.children)?
            .close()?
            .into_inner()
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
        let b = Run::new().add_text("Hello").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r>"#
        );
    }

    #[test]
    fn test_underline() {
        let b = Run::new().add_text("Hello").underline("single").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r><w:rPr><w:u w:val="single" /></w:rPr><w:t xml:space="preserve">Hello</w:t></w:r>"#
        );
    }

    #[test]
    fn test_strike() {
        let b = Run::new().add_text("Hello").strike().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r><w:rPr><w:strike /></w:rPr><w:t xml:space="preserve">Hello</w:t></w:r>"#
        );
    }

    #[test]
    fn test_child_json() {
        let c = RunChild::Text(Text::new("Hello"));
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"type":"text","data":{"preserveSpace":true,"text":"Hello"}}"#
        );
    }

    #[test]
    fn test_run_json() {
        let run = Run {
            children: vec![
                RunChild::Tab(Tab::new()),
                RunChild::Text(Text::new("Hello")),
                RunChild::Break(Break::new(BreakType::Page)),
                RunChild::DeleteText(DeleteText::new("deleted")),
            ],
            run_property: RunProperty {
                sz: Some(Sz::new(30)),
                sz_cs: Some(SzCs::new(30)),
                color: Some(Color::new("C9211E")),
                highlight: Some(Highlight::new("yellow")),
                underline: Some(Underline::new("single")),
                bold: Some(Bold::new()),
                bold_cs: Some(BoldCs::new()),
                italic: Some(Italic::new()),
                italic_cs: Some(ItalicCs::new()),
                vanish: Some(Vanish::new()),
                character_spacing: Some(CharacterSpacing::new(100)),
                ..RunProperty::default()
            },
        };
        assert_eq!(
            serde_json::to_string(&run).unwrap(),
            r#"{"runProperty":{"sz":30,"szCs":30,"color":"C9211E","highlight":"yellow","underline":"single","bold":true,"boldCs":true,"italic":true,"italicCs":true,"vanish":true,"characterSpacing":100},"children":[{"type":"tab"},{"type":"text","data":{"preserveSpace":true,"text":"Hello"}},{"type":"break","data":{"breakType":"page"}},{"type":"deleteText","data":{"text":"deleted","preserveSpace":true}}]}"#,
        );
    }

    #[test]
    fn test_run_footnote_reference() {
        let c = RunChild::FootnoteReference(FootnoteReference::new(1));
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"type":"footnoteReference","data":{"id":1}}"#
        );
    }

    #[test]
    fn test_run_shading() {
        let c = RunChild::Shading(Shading::new());
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"type":"shading","data":{"shdType":"clear","color":"auto","fill":"FFFFFF"}}"#
        );
    }
}
