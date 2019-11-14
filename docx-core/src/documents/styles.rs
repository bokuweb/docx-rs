use super::{DocDefaults, Style};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug)]
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
        let normal = Style::new("Normal", "Normal", StyleType::Paragraph);
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
        let c = Styles::new().add_style(Style::new("Title", "TitleName", StyleType::Paragraph));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:styles xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" mc:Ignorable="w14 w15"><w:docDefaults><w:rPrDefault><w:rPr /></w:rPrDefault></w:docDefaults><w:style w:type="paragraph" w:styleId="Normal"><w:name w:val="Normal" /><w:rPr /><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:basedOn w:val="Normal" /><w:next w:val="Normal" /><w:qFormat /></w:style><w:style w:type="paragraph" w:styleId="Title"><w:name w:val="TitleName" /><w:rPr /><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:basedOn w:val="Normal" /><w:next w:val="Normal" /><w:qFormat /></w:style></w:styles>"#
        );
    }
}
