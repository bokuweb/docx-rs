use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct SectionProperty {
    page_size: PageSize,
    page_margin: PageMargin,
    columns: usize,
    document_grid: usize,
}

impl SectionProperty {
    pub fn new() -> SectionProperty {
        Default::default()
    }
}

impl Default for SectionProperty {
    fn default() -> Self {
        Self {
            page_size: PageSize::new(),
            page_margin: PageMargin::new(),
            columns: 425,
            document_grid: 360,
        }
    }
}

impl BuildXML for SectionProperty {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_section_property()
            .add_child(&self.page_size)
            .add_child(&self.page_margin)
            .columns(&format!("{}", &self.columns))
            .document_grid("lines", &format!("{}", &self.document_grid))
            .close()
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
    fn test_section_property_default() {
        let c = SectionProperty::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" />
  <w:docGrid w:type="lines" w:linePitch="360" />
</w:sectPr>"#
        );
    }
}
