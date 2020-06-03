use crate::documents::{BuildXML, LevelJc, LevelText, NumberFormat, ParagraphProperty, Start};
use crate::types::*;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub level: usize,
    pub start: Start,
    pub format: NumberFormat,
    pub text: LevelText,
    pub jc: LevelJc,
    pub pstyle: Option<String>,
    pub paragraph_property: ParagraphProperty,
    pub suffix: LevelSuffixType,
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
            pstyle: None,
            paragraph_property: ParagraphProperty::new(),
            suffix: LevelSuffixType::Tab,
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
        self.pstyle = Some(style_id.into());
        self
    }

    pub fn suffix(mut self, s: LevelSuffixType) -> Self {
        self.suffix = s;
        self
    }
}

impl BuildXML for Level {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new()
            .open_level(&format!("{}", self.level))
            .add_child(&self.start)
            .add_child(&self.format)
            .add_child(&self.text)
            .add_child(&self.jc)
            .add_child(&self.paragraph_property);

        if self.suffix != LevelSuffixType::Tab {
            b = b.suffix(&self.suffix.to_string());
        }

        b.close().build()
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
            r#"<w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /></w:pPr></w:lvl>"#
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
            r#"<w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="320" w:right="0" w:hanging="200" /></w:pPr></w:lvl>"#
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
            r#"<w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /></w:pPr><w:suff w:val="space" />
</w:lvl>"#
        );
    }
}
