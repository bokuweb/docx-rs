use super::*;

use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

/*
17.9.8 lvlOverride (Numbering Level Definition Override)
This element specifies an optional override which shall be applied in place of zero or more levels from the abstract numbering definition for a given numbering definition instance. Each instance of this element is used to override the appearance and behavior of a given numbering level definition within the given abstract numbering definition.
*/
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelOverride {
    pub level: usize,
    pub override_start: Option<usize>,
    pub override_level: Option<Level>,
}

impl LevelOverride {
    pub fn new(level: usize) -> LevelOverride {
        LevelOverride {
            level,
            override_start: None,
            override_level: None,
        }
    }

    pub fn start(mut self, start: usize) -> LevelOverride {
        self.override_start = Some(start);
        self
    }

    pub fn level(mut self, override_level: Level) -> LevelOverride {
        self.override_level = Some(override_level);
        self
    }
}

impl BuildXML for LevelOverride {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b.open_level_override(&format!("{}", self.level));

        b = b.add_optional_child(&self.override_level);

        if let Some(start) = self.override_start {
            b = b.start_override(&format!("{}", start));
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
    fn test_level_override() {
        let c = LevelOverride::new(1).start(2);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:lvlOverride w:ilvl="1">
  <w:startOverride w:val="2" />
</w:lvlOverride>"#
        );
    }

    #[test]
    fn test_override_with_lvl() {
        let lvl = Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%4."),
            LevelJc::new("left"),
        );
        let c = LevelOverride::new(1).level(lvl);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:lvlOverride w:ilvl="1"><w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:rPr /></w:pPr><w:rPr /></w:lvl></w:lvlOverride>"#
        );
    }
}
