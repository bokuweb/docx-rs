use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Numbering {
    id: usize,
    abstract_num_id: usize,
}

impl Numbering {
    pub fn new(id: usize, abstract_num_id: usize) -> Self {
        Self {
            id,
            abstract_num_id,
        }
    }
}

impl BuildXML for Numbering {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let id = format!("{}", self.id);
        let abs_id = format!("{}", self.abstract_num_id);
        b.open_num(&id).abstract_num_id(&abs_id).close().build()
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
}
