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
    pub start: Option<usize>,
}

impl LevelOverride {
    pub fn new(level: usize) -> LevelOverride {
        LevelOverride { level, start: None }
    }

    pub fn start(mut self, start: usize) -> LevelOverride {
        self.start = Some(start);
        self
    }
}

impl BuildXML for LevelOverride {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b.open_level_override(&format!("{}", self.level));

        if let Some(start) = self.start {
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
}

// Example
/*
<w:num w:numId="5">
  <w:abstractNumId w:val="0"/>
  <w:lvlOverride w:ilvl="0">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
  <w:lvlOverride w:ilvl="1">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
  <w:lvlOverride w:ilvl="2">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
  <w:lvlOverride w:ilvl="3">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
  <w:lvlOverride w:ilvl="4">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
  <w:lvlOverride w:ilvl="5">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
  <w:lvlOverride w:ilvl="6">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
  <w:lvlOverride w:ilvl="7">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
  <w:lvlOverride w:ilvl="8">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
</w:num>
*/
