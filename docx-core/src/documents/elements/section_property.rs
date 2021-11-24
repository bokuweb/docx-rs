use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionProperty {
    page_size: PageSize,
    page_margin: PageMargin,
    columns: usize,
    doc_grid: DocGrid,
    header_reference: HeaderReference,
    footer_reference: Option<FooterReference>,
    section_type: Option<SectionType>,
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
            header_reference: HeaderReference::default(),
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
            .add_child(&self.header_reference)
            .columns(&format!("{}", &self.columns))
            .add_child(&self.doc_grid)
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
            r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:headerReference w:type="default" r:id="rId4" /><w:cols w:space="425" /><w:docGrid w:type="lines" w:linePitch="360" /></w:sectPr>"#
        );
    }

    #[test]
    fn test_section_property_with_footer() {
        let c = SectionProperty::new().footer_reference(FooterReference::new("default", "rId6"));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:headerReference w:type="default" r:id="rId4" /><w:cols w:space="425" /><w:docGrid w:type="lines" w:linePitch="360" /><w:footerReference w:type="default" r:id="rId6" /></w:sectPr>"#
        );
    }
}
