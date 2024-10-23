use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;
use std::io::Write;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageSize {
    w: u32,
    h: u32,
    orient: Option<PageOrientationType>,
}

// These values were based on microsoft office word2019 windows edition.
// <w:pgSz w:w="11906" w:h="16838"/>
impl Default for PageSize {
    fn default() -> PageSize {
        PageSize {
            w: 11906,
            h: 16838,
            orient: None,
        }
    }
}

impl PageSize {
    pub fn new() -> PageSize {
        Default::default()
    }

    pub fn size(self, w: u32, h: u32) -> PageSize {
        PageSize {
            w,
            h,
            orient: self.orient,
        }
    }

    pub fn width(mut self, w: u32) -> PageSize {
        self.w = w;
        self
    }

    pub fn height(mut self, h: u32) -> PageSize {
        self.h = h;
        self
    }

    pub fn orient(mut self, o: PageOrientationType) -> PageSize {
        self.orient = Some(o);
        self
    }
}

impl BuildXML for PageSize {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        let w = format!("{}", self.w);
        let h = format!("{}", self.h);

        XMLBuilder::from(stream)
            .apply(|b| match self.orient {
                None => b.page_size(&w, &h),
                Some(orient) => b.page_size_with_orient(&w, &h, &orient.to_string()),
            })?
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
    fn test_page_size_default() {
        let b = PageSize::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pgSz w:w="11906" w:h="16838" />"#
        );
    }
}
