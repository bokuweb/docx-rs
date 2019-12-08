use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct LevelJc {
    val: String,
}

impl LevelJc {
    pub fn new(val: impl Into<String>) -> Self {
        Self { val: val.into() }
    }
}

impl BuildXML for LevelJc {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.level_justification(&self.val).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_level_jc() {
        let c = LevelJc::new("left");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:lvlJc w:val="left" />"#);
    }
}
