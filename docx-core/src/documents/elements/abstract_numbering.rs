use crate::documents::{BuildXML, Level};
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractNumbering {
    id: usize,
    style_link: Option<String>,
    num_style_link: Option<String>,
    levels: Vec<Level>,
}

impl AbstractNumbering {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            style_link: None,
            num_style_link: None,
            levels: vec![],
        }
    }

    pub fn add_level(mut self, level: Level) -> Self {
        self.levels.push(level);
        self
    }

    pub fn num_style_link(mut self, link: impl Into<String>) -> Self {
        self.num_style_link = Some(link.into());
        self
    }

    pub fn style_link(mut self, link: impl Into<String>) -> Self {
        self.style_link = Some(link.into());
        self
    }
}

impl BuildXML for AbstractNumbering {
    fn build(&self) -> Vec<u8> {
        let id = format!("{}", self.id);
        let mut b = XMLBuilder::new();
        b = b.open_abstract_num(&id);
        for l in &self.levels {
            b = b.add_child(l);
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use crate::documents::{Level, LevelJc, LevelText, NumberFormat, Start};
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_numbering() {
        let mut c = AbstractNumbering::new(0);
        c = c.add_level(Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%4."),
            LevelJc::new("left"),
        ));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:abstractNum w:abstractNumId="0"><w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /></w:pPr><w:rPr /></w:lvl></w:abstractNum>"#
        );
    }

    #[test]
    fn test_numbering_json() {
        let mut c = AbstractNumbering::new(0);
        c = c
            .add_level(Level::new(
                1,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%4."),
                LevelJc::new("left"),
            ))
            .num_style_link("style1");
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"id":0,"styleLink":null,"numStyleLink":"style1","levels":[{"level":1,"start":1,"format":"decimal","text":"%4.","jc":"left","paragraphProperty":{"runProperty":{},"tabs":[]},"runProperty":{},"suffix":"tab","pstyle":null,"levelRestart":null}]}"#,
        );
    }
}
