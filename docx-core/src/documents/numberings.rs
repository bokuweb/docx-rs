use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;
use std::io::Write;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Numberings {
    pub abstract_nums: Vec<AbstractNumbering>,
    pub numberings: Vec<Numbering>,
}

impl Numberings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_abstract_numbering(mut self, n: AbstractNumbering) -> Self {
        self.abstract_nums.push(n);
        self
    }

    pub fn add_numbering(mut self, n: Numbering) -> Self {
        self.numberings.push(n);
        self
    }
}

impl BuildXML for Numberings {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .declaration(Some(true))?
            .open_numbering()?
            .add_optional_child(
                &self
                    .abstract_nums
                    .iter()
                    .find(|a| a.id == 1)
                    .is_none()
                    .then_some(create_default_numbering()),
            )?
            .add_children(&self.abstract_nums)?
            .add_optional_child(
                &self
                    .numberings
                    .iter()
                    .find(|n| n.id == 1)
                    .is_none()
                    .then_some(Numbering::new(1, 1)),
            )?
            .add_children(&self.numberings)?
            .close()?
            .into_inner()
    }
}

fn create_default_numbering() -> AbstractNumbering {
    AbstractNumbering::new(1)
        .add_level(
            Level::new(
                0,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%1."),
                LevelJc::new("left"),
            )
            .indent(Some(420), Some(SpecialIndentType::Hanging(420)), None, None),
        )
        .add_level(
            Level::new(
                1,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("(%2)"),
                LevelJc::new("left"),
            )
            .indent(Some(840), Some(SpecialIndentType::Hanging(420)), None, None),
        )
        .add_level(
            Level::new(
                2,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%3"),
                LevelJc::new("left"),
            )
            .indent(
                Some(1260),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                3,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%4."),
                LevelJc::new("left"),
            )
            .indent(
                Some(1680),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                4,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("(%5)"),
                LevelJc::new("left"),
            )
            .indent(
                Some(2100),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                5,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%6"),
                LevelJc::new("left"),
            )
            .indent(
                Some(2520),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                6,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%7."),
                LevelJc::new("left"),
            )
            .indent(
                Some(2940),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                7,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("(%8)"),
                LevelJc::new("left"),
            )
            .indent(
                Some(3360),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                8,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%9"),
                LevelJc::new("left"),
            )
            .indent(
                Some(3780),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_numberings_default() {
        let c = Numberings::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:numbering xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:abstractNum w:abstractNumId="1"><w:lvl w:ilvl="0"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%1." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="420" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="(%2)" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="840" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="2"><w:start w:val="1" /><w:numFmt w:val="decimalEnclosedCircle" /><w:lvlText w:val="%3" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="1260" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="3"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="1680" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="4"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="(%5)" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="2100" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="5"><w:start w:val="1" /><w:numFmt w:val="decimalEnclosedCircle" /><w:lvlText w:val="%6" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="2520" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="6"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%7." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="2940" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="7"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="(%8)" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="3360" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="8"><w:start w:val="1" /><w:numFmt w:val="decimalEnclosedCircle" /><w:lvlText w:val="%9" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="3780" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl></w:abstractNum><w:num w:numId="1"><w:abstractNumId w:val="1" /></w:num></w:numbering>"#
        );
    }

    #[test]
    fn test_numberings_custom_abstract_numbering_replaces_default() {
        let mut c = AbstractNumbering::new(1);
        c = c.add_level(Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%4."),
            LevelJc::new("left"),
        ));
        let mut d = Numberings::new();
        d = d.add_abstract_numbering(c);
        let b = d.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:numbering xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:abstractNum w:abstractNumId="1"><w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /></w:pPr><w:rPr /></w:lvl></w:abstractNum><w:num w:numId="1"><w:abstractNumId w:val="1" /></w:num></w:numbering>"#
        );
    }

    #[test]
    fn test_numberings_custom_numbering_replaces_default() {
        let mut c = AbstractNumbering::new(2);
        c = c.add_level(Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%4."),
            LevelJc::new("left"),
        ));
        let mut d = Numberings::new();
        d = d.add_abstract_numbering(c);
        d = d.add_numbering(Numbering::new(1, 2));
        let b = d.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:numbering xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:abstractNum w:abstractNumId="1"><w:lvl w:ilvl="0"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%1." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="420" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="(%2)" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="840" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="2"><w:start w:val="1" /><w:numFmt w:val="decimalEnclosedCircle" /><w:lvlText w:val="%3" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="1260" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="3"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="1680" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="4"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="(%5)" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="2100" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="5"><w:start w:val="1" /><w:numFmt w:val="decimalEnclosedCircle" /><w:lvlText w:val="%6" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="2520" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="6"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%7." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="2940" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="7"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="(%8)" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="3360" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl><w:lvl w:ilvl="8"><w:start w:val="1" /><w:numFmt w:val="decimalEnclosedCircle" /><w:lvlText w:val="%9" /><w:lvlJc w:val="left" /><w:pPr><w:rPr /><w:ind w:left="3780" w:right="0" w:hanging="420" /></w:pPr><w:rPr /></w:lvl></w:abstractNum><w:abstractNum w:abstractNumId="2"><w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /></w:pPr><w:rPr /></w:lvl></w:abstractNum><w:num w:numId="1"><w:abstractNumId w:val="2" /></w:num></w:numbering>"#
        );
    }
}
