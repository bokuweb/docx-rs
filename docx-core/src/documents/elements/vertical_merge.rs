use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct VMerge {
    val: VMergeType,
}

impl VMerge {
    pub fn new(v: VMergeType) -> VMerge {
        VMerge { val: v }
    }
}

impl BuildXML for VMerge {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .vertical_merge(&self.val.to_string())
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
    fn test_build() {
        let b = VMerge::new(VMergeType::Continue).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:vMerge w:val="continue" />"#
        );
    }
}
