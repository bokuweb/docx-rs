use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;
use std::io::Write;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub level: usize,
    pub start: Start,
    pub format: NumberFormat,
    pub text: LevelText,
    pub jc: LevelJc,
    pub paragraph_property: ParagraphProperty,
    pub run_property: RunProperty,
    pub suffix: LevelSuffixType,
    pub pstyle: Option<ParagraphStyle>,
    pub level_restart: Option<LevelRestart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_lgl: Option<IsLgl>,
}

impl Level {
    pub fn new(
        level: usize,
        start: Start,
        format: NumberFormat,
        text: LevelText,
        jc: LevelJc,
    ) -> Level {
        Self {
            level,
            start,
            format,
            text,
            jc,
            paragraph_property: ParagraphProperty::new(),
            run_property: RunProperty::new(),
            suffix: LevelSuffixType::Tab,
            pstyle: None,
            level_restart: None,
            is_lgl: None,
        }
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

    pub fn paragraph_style(mut self, style_id: impl Into<String>) -> Self {
        self.pstyle = Some(ParagraphStyle::new(Some(style_id.into())));
        self
    }

    pub fn suffix(mut self, s: LevelSuffixType) -> Self {
        self.suffix = s;
        self
    }

    // run property
    pub fn size(mut self, size: usize) -> Self {
        self.run_property = self.run_property.size(size);
        self
    }

    pub fn spacing(mut self, v: i32) -> Self {
        self.run_property = self.run_property.spacing(v);
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

    pub fn fonts(mut self, f: RunFonts) -> Self {
        self.run_property = self.run_property.fonts(f);
        self
    }

    pub fn level_restart(mut self, v: u32) -> Self {
        self.level_restart = Some(LevelRestart::new(v));
        self
    }

    pub fn is_lgl(mut self) -> Self {
        self.is_lgl = Some(IsLgl::new());
        self
    }
}

impl BuildXML for Level {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_level(&format!("{}", self.level))?
            .add_child(&self.start)?
            .add_child(&self.format)?
            .add_child(&self.text)?
            .add_child(&self.jc)?
            .add_child(&self.paragraph_property)?
            .add_child(&self.run_property)?
            .add_optional_child(&self.pstyle)?
            .add_optional_child(&self.level_restart)?
            .add_optional_child(&self.is_lgl)?
            .apply_if(self.suffix != LevelSuffixType::Tab, |b| {
                b.suffix(&self.suffix.to_string())
            })?
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
    fn test_level() {
        let b = Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%4."),
            LevelJc::new("left"),
        )
        .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /></w:pPr><w:rPr /></w:lvl>"#
        );
    }

    #[test]
    fn test_level_indent() {
        let b = Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%4."),
            LevelJc::new("left"),
        )
        .indent(Some(320), Some(SpecialIndentType::Hanging(200)), None, None)
        .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="320" w:right="0" w:hanging="200" /></w:pPr><w:rPr /></w:lvl>"#
        );
    }
    #[test]
    fn test_level_with_suff() {
        let b = Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%4."),
            LevelJc::new("left"),
        )
        .suffix(LevelSuffixType::Space)
        .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /></w:pPr><w:rPr /><w:suff w:val="space" /></w:lvl>"#
        );
    }
    #[test]
    fn test_level_with_pstyle() {
        let b = Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%4."),
            LevelJc::new("left"),
        )
        .paragraph_style("a-style")
        .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /></w:pPr><w:rPr /><w:pStyle w:val="a-style" /></w:lvl>"#
        );
    }
}
