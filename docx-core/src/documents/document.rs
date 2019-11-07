use super::{DocDefaults, Style};
use crate::documents::BuildXML;
use crate::xml_builder::*;
use crate::StyleType;

pub struct Document {}

impl Document {
    pub fn new() -> Document {
        Default::default()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {}
    }
}

impl BuildXML for Document {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_document().open_body().close().close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_document() {
        let b = Document::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:document xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w10="urn:schemas-microsoft-com:office:word" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14 wp14">
  <w:body />
</w:document>"#
        );
    }
}
