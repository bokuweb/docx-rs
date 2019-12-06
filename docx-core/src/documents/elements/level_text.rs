use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct LevelText<'a> {
    val: &'a str,
}

impl<'a> LevelText<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}

impl<'a> BuildXML for LevelText<'a> {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.level_text(self.val).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_level_text() {
        let c = LevelText::new("%4.");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:lvlText w:val="%4." />"#);
    }
}
