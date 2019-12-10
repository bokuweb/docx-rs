use crate::documents::{BuildXML, LevelJc, LevelText, NumberFormat, ParagraphProperty, Start};
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Level {
    level: usize,
    start: Start,
    format: NumberFormat,
    text: LevelText,
    jc: LevelJc,
    pub paragraph_property: ParagraphProperty,
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
        }
    }

    pub fn indent(mut self, left: usize, special_indent: Option<SpecialIndentType>) -> Self {
        self.paragraph_property = self.paragraph_property.indent(left, special_indent);
        self
    }
}

impl BuildXML for Level {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_level(&format!("{}", self.level))
            .add_child(&self.start)
            .add_child(&self.format)
            .add_child(&self.text)
            .add_child(&self.jc)
            .add_child(&self.paragraph_property)
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
              r#"<w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr></w:lvl>"#
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
        .indent(320, Some(SpecialIndentType::Hanging(200)))
        .build();
        assert_eq!(
              str::from_utf8(&b).unwrap(),
              r#"<w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:pStyle w:val="Normal" /><w:rPr /><w:ind w:left="320" w:hanging="200" /></w:pPr></w:lvl>"#
          );
    }
}
