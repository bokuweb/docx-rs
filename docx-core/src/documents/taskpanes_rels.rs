use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct TaskpanesRels {
    pub rels: Vec<(String, String, String)>,
}

impl TaskpanesRels {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_rel(mut self) -> Self {
        let index = self.rels.len() + 1;
        self.rels.push((
            "http://schemas.microsoft.com/office/2011/relationships/webextension".to_string(),
            format!("rId{}", index),
            format!("webextension{}.xml", index),
        ));
        self
    }

    pub fn find_target(&self, rel_type: &str) -> Option<&(String, String, String)> {
        self.rels.iter().find(|rel| rel.0 == rel_type)
    }
}

impl BuildXML for TaskpanesRels {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .declaration(Some(true))?
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships")?
            .apply_each(&self.rels, |(k, id, v), b| b.relationship(id, k, v))?
            .close()?
            .into_inner()
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
        let c = TaskpanesRels::new().add_rel();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.microsoft.com/office/2011/relationships/webextension" Target="webextension1.xml" /></Relationships>"#
        );
    }
}
