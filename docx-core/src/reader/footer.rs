use std::io::Read;
use std::str::FromStr;

use crate::reader::*;
use xml::reader::{EventReader, XmlEvent};

use super::{Paragraph, Table};

impl FromXML for Footer {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut footer = Self::default();
        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Paragraph => {
                            let p = Paragraph::read(&mut parser, &attributes)?;
                            footer = footer.add_paragraph(p);
                            continue;
                        }
                        XMLElement::Table => {
                            let t = Table::read(&mut parser, &attributes)?;
                            footer = footer.add_table(t);
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndDocument) => break,
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
        Ok(footer)
    }
}

#[test]
fn test_footer_from_xml() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:ftr xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
    xmlns:o="urn:schemas-microsoft-com:office:office"
    xmlns:v="urn:schemas-microsoft-com:vml"
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:w10="urn:schemas-microsoft-com:office:word"
    xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"
    xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"
    xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
    xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing"
    xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14 wp14">
    <w:p w14:paraId="12345678">
        <w:pPr>
            <w:rPr />
        </w:pPr>
        <w:r>
            <w:rPr />
            <w:t xml:space="preserve">Hello Footer</w:t>
        </w:r>
    </w:p>
</w:ftr>"#;
    let h = Footer::from_xml(xml.as_bytes()).unwrap();
    let expected =
        Footer::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello Footer")));
    assert_eq!(h, expected)
}
