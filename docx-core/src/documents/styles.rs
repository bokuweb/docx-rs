use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Styles {
    doc_defaults: DocDefaults,
    styles: Vec<Style>,
}

impl Styles {
    pub fn new() -> Styles {
        Default::default()
    }

    pub fn add_style(mut self, style: Style) -> Self {
        self.styles.push(style);
        self
    }

    pub fn default_size(mut self, size: usize) -> Self {
        self.doc_defaults = self.doc_defaults.size(size);
        self
    }

    pub fn default_spacing(mut self, spacing: i32) -> Self {
        self.doc_defaults = self.doc_defaults.spacing(spacing);
        self
    }

    pub fn default_fonts(mut self, font: RunFonts) -> Self {
        self.doc_defaults = self.doc_defaults.fonts(font);
        self
    }

    pub(crate) fn doc_defaults(mut self, doc_defaults: DocDefaults) -> Self {
        self.doc_defaults = doc_defaults;
        self
    }

    pub fn find_style_by_id(&self, id: &str) -> Option<&Style> {
        self.styles.iter().find(|s| s.style_id == id)
    }

    pub fn create_heading_style_map(&self) -> std::collections::HashMap<String, usize> {
        self.styles
            .iter()
            .filter_map(|s| {
                if s.name.is_heading() {
                    let n = s.name.get_heading_number();
                    n.map(|n| (s.style_id.clone(), n))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            doc_defaults: DocDefaults::new(),
            styles: vec![],
        }
    }
}

impl BuildXML for Styles {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let normal = Style::new("Normal", StyleType::Paragraph).name("Normal");
        b.open_styles()
            .add_child(&self.doc_defaults)
            .add_child(&normal)
            .add_children(&self.styles)
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::StyleType;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_style() {
        let c =
            Styles::new().add_style(Style::new("Title", StyleType::Paragraph).name("TitleName"));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:styles xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" mc:Ignorable="w14 w15"><w:docDefaults><w:rPrDefault><w:rPr /></w:rPrDefault></w:docDefaults><w:style w:type="paragraph" w:styleId="Normal"><w:name w:val="Normal" /><w:rPr /><w:pPr><w:rPr /></w:pPr><w:qFormat /></w:style><w:style w:type="paragraph" w:styleId="Title"><w:name w:val="TitleName" /><w:rPr /><w:pPr><w:rPr /></w:pPr><w:qFormat /></w:style></w:styles>"#
        );
    }

    #[test]
    fn test_table_style(){
        let c =
            Styles::new().add_style(Style::new("Table", StyleType::Table).name("Table Style").table_property(TableProperty::new().set_margins(TableCellMargins::new().margin_left(108, WidthType::Dxa).margin_right(108,WidthType::Dxa))));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:styles xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" mc:Ignorable="w14 w15"><w:docDefaults><w:rPrDefault><w:rPr /></w:rPrDefault></w:docDefaults><w:style w:type="paragraph" w:styleId="Normal"><w:name w:val="Normal" /><w:rPr /><w:pPr><w:rPr /></w:pPr><w:qFormat /></w:style><w:style w:type="table" w:styleId="Table"><w:name w:val="Table Style" /><w:rPr /><w:pPr><w:rPr /></w:pPr><w:tcPr /><w:tblPr><w:tblW w:w="0" w:type="dxa" /><w:jc w:val="left" /><w:tblBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:right w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tblBorders><w:tblCellMar>
  <w:top w:w="0" w:type="dxa" />
  <w:left w:w="108" w:type="dxa" />
  <w:bottom w:w="0" w:type="dxa" />
  <w:right w:w="108" w:type="dxa" />
</w:tblCellMar></w:tblPr><w:qFormat /></w:style></w:styles>"#
        );
    }

    #[test]
    fn test_heading_style() {
        let c = Styles::new().add_style(Style::new("ToC", StyleType::Paragraph).name("heading 3"));
        let mut m = std::collections::HashMap::new();
        m.insert("ToC".to_string(), 3);
        let b = c.create_heading_style_map();
        assert_eq!(b, m);
    }
}
