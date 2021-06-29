use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TaskpanesRels {
    pub rels: Vec<(String, String, String)>,
}

impl TaskpanesRels {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_default(mut self) -> Self {
        self.rels.push((
            "http://schemas.microsoft.com/office/2011/relationships/webextension".to_owned(),
            "rId1".to_owned(),
            "webextension1.xml".to_owned(),
        ));
        self
    }

    pub fn add_rel(
        mut self,
        id: impl Into<String>,
        rel_type: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        self.rels.push((rel_type.into(), id.into(), target.into()));
        self
    }

    pub fn find_target(&self, rel_type: &str) -> Option<&(String, String, String)> {
        self.rels.iter().find(|rel| rel.0 == rel_type)
    }
}

impl Default for TaskpanesRels {
    fn default() -> Self {
        TaskpanesRels { rels: Vec::new() }
    }
}

impl BuildXML for TaskpanesRels {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b
            .declaration(None)
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships");
        for (k, id, v) in self.rels.iter() {
            b = b.relationship(id, k, v);
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
    fn test_build() {
        let c = TaskpanesRels::new().set_default();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.microsoft.com/office/2011/relationships/webextension" Target="webextension1.xml" />
</Relationships>"#
        );
    }
}
