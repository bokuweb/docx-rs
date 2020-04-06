use super::*;
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WpsTextBox {
    pub children: Vec<TextBoxContent>,
    pub has_numbering: bool,
}

impl WpsTextBox {
    pub fn new() -> WpsTextBox {
        Default::default()
    }

    pub fn add_content(mut self, c: TextBoxContent) -> Self {
        if c.has_numbering {
            self.has_numbering = true
        }
        self.children.push(c);
        self
    }
}

impl Default for WpsTextBox {
    fn default() -> Self {
        WpsTextBox {
            children: vec![],
            has_numbering: false,
        }
    }
}

impl BuildXML for WpsTextBox {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_wp_text_box();
        for c in &self.children {
            b = b.add_child(c);
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
    fn test_wp_text_box_build() {
        let c = TextBoxContent::new().add_paragraph(Paragraph::new());
        let b = WpsTextBox::new().add_content(c).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<wps:txbx><w:txbxContent><w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr></w:p></w:txbxContent></wps:txbx>"#
        );
    }
}
