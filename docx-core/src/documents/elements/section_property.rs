use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;
use crate::{Footer, Header};

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionProperty {
    pub page_size: PageSize,
    pub page_margin: PageMargin,
    pub columns: usize,
    pub space: usize,
    pub title_pg: bool,
    pub text_direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_grid: Option<DocGrid>,
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
    pub footer: Option<Footer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_footer_reference: Option<FooterReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_footer: Option<Footer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_footer_reference: Option<FooterReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_footer: Option<Footer>,
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
        self.doc_grid = Some(doc_grid);
        self
    }

    pub fn text_direction(mut self, direction: String) -> Self {
        self.text_direction = direction;
        self
    }

    pub fn title_pg(mut self) -> Self {
        self.title_pg = true;
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
        self.title_pg = true;
        self
    }

    pub fn first_header_without_title_pg(mut self, h: Header, rid: &str) -> Self {
        self.first_header_reference = Some(HeaderReference::new("first", rid));
        self.first_header = Some(h);
        self
    }

    pub fn even_header(mut self, h: Header, rid: &str) -> Self {
        self.even_header_reference = Some(HeaderReference::new("even", rid));
        self.even_header = Some(h);
        self
    }

    pub fn footer(mut self, h: Footer, rid: &str) -> Self {
        self.footer_reference = Some(FooterReference::new("default", rid));
        self.footer = Some(h);
        self
    }

    pub fn first_footer(mut self, h: Footer, rid: &str) -> Self {
        self.first_footer_reference = Some(FooterReference::new("first", rid));
        self.first_footer = Some(h);
        self.title_pg = true;
        self
    }

    pub fn first_footer_without_title_pg(mut self, h: Footer, rid: &str) -> Self {
        self.first_footer_reference = Some(FooterReference::new("first", rid));
        self.first_footer = Some(h);
        self
    }

    pub fn even_footer(mut self, h: Footer, rid: &str) -> Self {
        self.even_footer_reference = Some(FooterReference::new("even", rid));
        self.even_footer = Some(h);
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

    pub fn get_footers(&self) -> Vec<&Footer> {
        let mut footers = vec![];
        if let Some(ref footer) = self.footer {
            footers.push(footer);
        }
        if let Some(ref footer) = self.first_footer {
            footers.push(footer);
        }
        if let Some(ref footer) = self.even_footer {
            footers.push(footer);
        }
        footers
    }
}

impl Default for SectionProperty {
    fn default() -> Self {
        Self {
            page_size: PageSize::new(),
            page_margin: PageMargin::new(),
            columns: 1,
            space: 425,
            title_pg: false,
            text_direction: "lrTb".to_string(),
            doc_grid: None,
            // headers
            header_reference: None,
            header: None,
            first_header_reference: None,
            first_header: None,
            even_header_reference: None,
            even_header: None,
            // footers
            footer_reference: None,
            footer: None,
            first_footer_reference: None,
            first_footer: None,
            even_footer_reference: None,
            even_footer: None,
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
            .columns(&format!("{}", &self.space), &format!("{}", &self.columns))
            .add_optional_child(&self.doc_grid)
            .add_optional_child(&self.header_reference)
            .add_optional_child(&self.first_header_reference)
            .add_optional_child(&self.even_header_reference)
            .add_optional_child(&self.footer_reference)
            .add_optional_child(&self.first_footer_reference)
            .add_optional_child(&self.even_footer_reference);
        if !self.text_direction.eq("lrTb") {
            b = b.text_direction(&self.text_direction);
        }
        if let Some(t) = self.section_type {
            b = b.type_tag(&t.to_string());
        }

        if self.title_pg {
            b = b.title_pg();
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
    fn text_section_text_direction() {
        let mut c = SectionProperty::new();
        c = c.text_direction("tbRl".to_string());
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" w:num="1" />
  <w:textDirection w:val="tbRl" />
</w:sectPr>"#
        )
    }

    #[test]
    fn test_section_property_default() {
        let c = SectionProperty::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" w:num="1" />
</w:sectPr>"#
        );
    }

    #[test]
    fn test_section_property_with_footer() {
        let c = SectionProperty::new().footer(Footer::new(), "rId6");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" w:num="1" /><w:footerReference w:type="default" r:id="rId6" /></w:sectPr>"#
        );
    }

    #[test]
    fn test_section_property_with_title_pf() {
        let c = SectionProperty::new().title_pg();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" w:num="1" />
  <w:titlePg />
</w:sectPr>"#
        );
    }
}
