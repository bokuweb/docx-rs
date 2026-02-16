use super::*;
use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
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

impl BuildXML for WpsTextBox {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_wp_text_box()?
            .add_children(&self.children)?
            .close()?
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
    fn test_wp_text_box_build() {
        let c = TextBoxContent::new().add_paragraph(Paragraph::new());
        let b = WpsTextBox::new().add_content(c).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<wps:txbx><w:txbxContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr></w:p></w:txbxContent></wps:txbx>"#
        );
    }
}
