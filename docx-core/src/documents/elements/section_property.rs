use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;
use crate::Header;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionProperty {
    pub page_size: PageSize,
    pub page_margin: PageMargin,
    pub columns: usize,
    pub doc_grid: DocGrid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_reference: Option<HeaderReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Header>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_header_reference: Option<HeaderReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_header: Option<Header>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_header_reference: Option<HeaderReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_header: Option<Header>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_reference: Option<FooterReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_type: Option<SectionType>,
}

impl SectionProperty {
    pub fn new() -> SectionProperty {
        Default::default()
    }

    pub fn page_size(mut self, size: PageSize) -> Self {
        self.page_size = size;
        self
    }

    pub fn page_margin(mut self, margin: PageMargin) -> Self {
        self.page_margin = margin;
        self
    }

    pub fn page_orient(mut self, o: PageOrientationType) -> Self {
        self.page_size = self.page_size.orient(o);
        self
    }

    pub fn doc_grid(mut self, doc_grid: DocGrid) -> Self {
        self.doc_grid = doc_grid;
        self
    }

    pub fn header(mut self, h: Header, rid: &str) -> Self {
        self.header_reference = Some(HeaderReference::new("default", rid));
        self.header = Some(h);
        self
    }

    pub fn first_header(mut self, h: Header, rid: &str) -> Self {
        self.first_header_reference = Some(HeaderReference::new("first", rid));
        self.first_header = Some(h);
        self
    }

    pub fn even_header(mut self, h: Header, rid: &str) -> Self {
        self.even_header_reference = Some(HeaderReference::new("even", rid));
        self.even_header = Some(h);
        self
    }

    pub fn get_headers(&self) -> Vec<&Header> {
        let mut headers = vec![];
        if let Some(ref header) = self.header {
            headers.push(header);
        }
        if let Some(ref header) = self.first_header {
            headers.push(header);
        }
        if let Some(ref header) = self.even_header {
            headers.push(header);
        }
        headers
    }

    pub fn footer_reference(mut self, r: FooterReference) -> Self {
        self.footer_reference = Some(r);
        self
    }
}

impl Default for SectionProperty {
    fn default() -> Self {
        Self {
            page_size: PageSize::new(),
            page_margin: PageMargin::new(),
            columns: 425,
            doc_grid: DocGrid::default(),
            header_reference: None,
            header: None,
            first_header_reference: None,
            first_header: None,
            even_header_reference: None,
            even_header: None,
            footer_reference: None,
            section_type: None,
        }
    }
}

impl BuildXML for SectionProperty {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b
            .open_section_property()
            .add_child(&self.page_size)
            .add_child(&self.page_margin)
            .columns(&format!("{}", &self.columns))
            .add_child(&self.doc_grid)
            .add_optional_child(&self.header_reference)
            .add_optional_child(&self.first_header_reference)
            .add_optional_child(&self.even_header_reference)
            .add_optional_child(&self.footer_reference);

        if let Some(t) = self.section_type {
            b = b.type_tag(&t.to_string());
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
    fn test_section_property_default() {
        let c = SectionProperty::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" /><w:docGrid w:type="lines" w:linePitch="360" /></w:sectPr>"#
        );
    }

    #[test]
    fn test_section_property_with_footer() {
        let c = SectionProperty::new().footer_reference(FooterReference::new("default", "rId6"));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" /><w:docGrid w:type="lines" w:linePitch="360" /><w:footerReference w:type="default" r:id="rId6" /></w:sectPr>"#
        );
    }
}
