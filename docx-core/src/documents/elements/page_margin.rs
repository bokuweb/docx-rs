use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct PageMargin {
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
    header: usize,
    footer: usize,
    gutter: usize,
}

// These values were based on microsoft office word2019 windows edition.
// <w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0"/>
impl Default for PageMargin {
    fn default() -> PageMargin {
        PageMargin {
            top: 1985,
            left: 1701,
            bottom: 1701,
            right: 1701,
            header: 851,
            footer: 992,
            gutter: 0,
        }
    }
}

impl PageMargin {
    pub fn new() -> PageMargin {
        Default::default()
    }

    pub fn top(self, v: usize) -> PageMargin {
        PageMargin { top: v, ..self }
    }

    pub fn left(self, v: usize) -> PageMargin {
        PageMargin { left: v, ..self }
    }

    pub fn bottom(self, v: usize) -> PageMargin {
        PageMargin { bottom: v, ..self }
    }

    pub fn right(self, v: usize) -> PageMargin {
        PageMargin { right: v, ..self }
    }

    pub fn header(self, v: usize) -> PageMargin {
        PageMargin { header: v, ..self }
    }

    pub fn gutter(self, v: usize) -> PageMargin {
        PageMargin { gutter: v, ..self }
    }
}

impl BuildXML for PageMargin {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .page_margin(
                &format!("{}", self.top),
                &format!("{}", self.right),
                &format!("{}", self.bottom),
                &format!("{}", self.left),
                &format!("{}", self.header),
                &format!("{}", self.footer),
                &format!("{}", self.gutter),
            )
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
    fn test_page_margin_default() {
        let b = PageMargin::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" />"#
        );
    }
}
