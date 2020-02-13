use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::reader::{FromXML, ReaderError};

use std::str::FromStr;

impl FromXML for Styles {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut styles = Self::default();
        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Style = e {
                        let s = Style::read(&mut parser, &attributes)?;
                        styles = styles.add_style(s);
                        continue;
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Styles = e {
                        break;
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
        Ok(styles)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_xml() {
        let xml = r#"<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:style w:type="character" w:styleId="FootnoteTextChar">
        <w:name w:val="Footnote Text Char"></w:name>
        <w:rPr>
            <w:sz w:val="20"></w:sz>
            <w:szCs w:val="20"></w:szCs>
        </w:rPr>
        <w:uiPriority w:val="99"></w:uiPriority>
        <w:unhideWhenUsed></w:unhideWhenUsed>
        <w:basedOn w:val="DefaultParagraphFont"></w:basedOn>
        <w:link w:val="FootnoteText"></w:link>
        <w:uiPriority w:val="99"></w:uiPriority>
        <w:semiHidden></w:semiHidden>
    </w:style>
</w:styles>"#;
        let s = Styles::from_xml(xml.as_bytes()).unwrap();
        let mut styles = Styles::new();
        styles = styles.add_style(
            Style::new("FootnoteTextChar", StyleType::Character)
                .name("Footnote Text Char")
                .size(20),
        );
        assert_eq!(s, styles);
    }
}
