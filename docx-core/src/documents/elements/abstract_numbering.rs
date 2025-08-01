use crate::documents::{BuildXML, Level};
use crate::xml_builder::*;
use std::io::Write;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractNumbering {
    pub id: usize,
    pub style_link: Option<String>,
    pub num_style_link: Option<String>,
    pub levels: Vec<Level>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub multi_level_type: Option<String>,
}

impl AbstractNumbering {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            style_link: None,
            num_style_link: None,
            levels: vec![],
            multi_level_type: None,
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
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        let mut builder = XMLBuilder::from(stream)  
            .open_abstract_num(&self.id.to_string())?;  
          
        // 添加 multiLevelType 元素（如果存在）  
        if let Some(ref multi_level_type) = self.multi_level_type {  
            builder = builder.multi_level_type(multi_level_type)?;  
        }  
          
        builder  
            .add_children(&self.levels)?  
            .close()?  
            .into_inner()  
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
