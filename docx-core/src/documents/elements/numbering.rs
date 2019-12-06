use crate::documents::{BuildXML, Level};
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Numbering<'a> {
    id: &'a str,
    levels: Vec<Level<'a>>,
}

impl<'a> Numbering<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { id, levels: vec![] }
    }

    pub fn add_level(mut self, level: Level<'a>) -> Self {
        self.levels.push(level);
        self
    }
}

impl<'a> BuildXML for Numbering<'a> {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b.open_abstract_num(self.id);
        for l in &self.levels {
            b = b.add_child(l);
        }
        b.close()
            .open_num(self.id)
            .abstract_num_id(self.id)
            .close()
            .build()
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
        let mut c = Numbering::new("0");
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
            r#"<w:abstractNum w:abstractNumId="0"><w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr></w:lvl></w:abstractNum>
<w:num w:numId="0">
  <w:abstractNumId w:val="0" />
</w:num>"#
        );
    }
}
