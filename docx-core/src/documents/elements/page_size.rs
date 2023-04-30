use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

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
        if let Some(orient) = self.orient {
            if orient != o {
                self.w, self.h = self.h, self.w;
            }
        }
        
        self.orient = Some(o);
        self
    }
}

impl BuildXML for PageSize {
    fn build(&self) -> Vec<u8> {
        if let Some(orient) = self.orient {
            XMLBuilder::new()
                .page_size_with_orient(
                    &format!("{}", self.w),
                    &format!("{}", self.h),
                    &orient.to_string(),
                )
                .build()
        } else {
            XMLBuilder::new()
                .page_size(&format!("{}", self.w), &format!("{}", self.h))
                .build()
        }
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
