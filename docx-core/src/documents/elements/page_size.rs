use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct PageSize {
    w: usize,
    h: usize,
}

// These values were based on microsoft office word2019 windows edition.
// <w:pgSz w:w="11906" w:h="16838"/>
impl Default for PageSize {
    fn default() -> PageSize {
        PageSize { w: 11906, h: 16838 }
    }
}

impl PageSize {
    pub fn new() -> PageSize {
        Default::default()
    }

    pub fn size(self, w: usize, h: usize) -> PageSize {
        PageSize { w, h }
    }
}

impl BuildXML for PageSize {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .page_size(&format!("{}", self.w), &format!("{}", self.h))
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
    fn test_page_size_default() {
        let b = PageSize::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pgSz w:w="11906" w:h="16838" />"#
        );
    }
}
