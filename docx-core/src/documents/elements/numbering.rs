use crate::documents::BuildXML;
use crate::xml_builder::*;

use super::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Numbering {
    id: usize,
    abstract_num_id: usize,
    pub level_overrides: Vec<LevelOverride>,
}

impl Numbering {
    pub fn new(id: usize, abstract_num_id: usize) -> Self {
        Self {
            id,
            abstract_num_id,
            level_overrides: vec![],
        }
    }

    pub fn overrides(mut self, overrides: Vec<LevelOverride>) -> Self {
        self.level_overrides = overrides;
        self
    }

    pub fn add_override(mut self, o: LevelOverride) -> Self {
        self.level_overrides.push(o);
        self
    }
}

impl BuildXML for Numbering {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let id = format!("{}", self.id);
        let abs_id = format!("{}", self.abstract_num_id);
        b.open_num(&id)
            .abstract_num_id(&abs_id)
            .add_children(&self.level_overrides)
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
    fn test_numbering() {
        let c = Numbering::new(0, 2);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:num w:numId="0">
  <w:abstractNumId w:val="2" />
</w:num>"#
        );
    }
    #[test]
    fn test_numbering_override() {
        let c = Numbering::new(0, 2);
        let overrides = vec![
            LevelOverride::new(0).start(1),
            LevelOverride::new(1).start(1),
        ];
        let b = c.overrides(overrides).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:num w:numId="0">
  <w:abstractNumId w:val="2" /><w:lvlOverride w:ilvl="0">
  <w:startOverride w:val="1" />
</w:lvlOverride><w:lvlOverride w:ilvl="1">
  <w:startOverride w:val="1" />
</w:lvlOverride></w:num>"#
        );
    }

    #[test]
    fn test_numbering_override_json() {
        let c = Numbering::new(0, 2);
        let overrides = vec![
            LevelOverride::new(0).start(1),
            LevelOverride::new(1).start(1),
        ];
        assert_eq!(
            serde_json::to_string(&c.overrides(overrides)).unwrap(),
            r#"{"id":0,"abstractNumId":2,"levelOverrides":[{"level":0,"overrideStart":1,"overrideLevel":null},{"level":1,"overrideStart":1,"overrideLevel":null}]}"#
        );
    }
}
