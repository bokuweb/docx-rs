use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::reader::{FromXML, ReaderError};

use std::str::FromStr;

impl FromXML for Numberings {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut nums = Self::default();
        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::AbstractNumbering => {
                            let mut id = 0;
                            for a in attributes {
                                let local_name = &a.name.local_name;
                                if local_name == "abstractNumId" {
                                    id = usize::from_str(&a.value)?;
                                }
                            }
                            let mut abs_num = AbstractNumbering::new(id);
                            loop {
                                let e = parser.next();
                                match e {
                                    Ok(XmlEvent::StartElement {
                                        attributes, name, ..
                                    }) => {
                                        let e = XMLElement::from_str(&name.local_name).unwrap();
                                        match e {
                                            XMLElement::Level => {
                                                let l = Level::read(&mut parser, &attributes)?;
                                                abs_num = abs_num.add_level(l);
                                            }
                                            XMLElement::StyleLink => {
                                                abs_num = abs_num.style_link(&attributes[0].value)
                                            }
                                            XMLElement::NumStyleLink => {
                                                abs_num =
                                                    abs_num.num_style_link(&attributes[0].value)
                                            }
                                            _ => {}
                                        }
                                    }
                                    Ok(XmlEvent::EndElement { name, .. }) => {
                                        let e = XMLElement::from_str(&name.local_name).unwrap();
                                        if let XMLElement::AbstractNumbering = e {
                                            nums = nums.add_abstract_numbering(abs_num);
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            continue;
                        }
                        XMLElement::Num => {
                            let mut id = 0;
                            for a in attributes {
                                let local_name = &a.name.local_name;
                                if local_name == "numId" {
                                    id = usize::from_str(&a.value)?;
                                }
                            }
                            let mut abs_num_id = 0;
                            let mut level_overrides = vec![];

                            loop {
                                let e = parser.next();
                                match e {
                                    Ok(XmlEvent::StartElement {
                                        attributes, name, ..
                                    }) => {
                                        let e = XMLElement::from_str(&name.local_name).unwrap();
                                        match e {
                                            XMLElement::AbstractNumberingId => {
                                                abs_num_id = usize::from_str(&attributes[0].value)?
                                            }
                                            XMLElement::LvlOverride => {
                                                let o =
                                                    LevelOverride::read(&mut parser, &attributes)?;
                                                level_overrides.push(o);
                                            }
                                            _ => {}
                                        }
                                    }
                                    Ok(XmlEvent::EndElement { name, .. }) => {
                                        let e = XMLElement::from_str(&name.local_name).unwrap();
                                        if let XMLElement::Num = e {
                                            let num = Numbering::new(id, abs_num_id);
                                            nums =
                                                nums.add_numbering(num.overrides(level_overrides));
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Numbering = e {
                        break;
                    }
                }
                Ok(XmlEvent::EndDocument { .. }) => break,
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
        Ok(nums)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_numberings_from_xml() {
        let xml = r#"<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" >
    <w:abstractNum w:abstractNumId="0" w15:restartNumberingAfterBreak="0">
        <w:multiLevelType w:val="hybridMultilevel"></w:multiLevelType>
        <w:lvl w:ilvl="0" w15:tentative="1">
            <w:start w:val="1"></w:start>
            <w:numFmt w:val="bullet"></w:numFmt>
            <w:lvlText w:val="●"></w:lvlText>
            <w:lvlJc w:val="left"></w:lvlJc>
            <w:pPr>
                <w:ind w:left="720" w:hanging="360"></w:ind>
            </w:pPr>
            <w:rPr></w:rPr>
        </w:lvl>
    </w:abstractNum>
    <w:num w:numId="1">
        <w:abstractNumId w:val="0"></w:abstractNumId>
    </w:num>
</w:numbering>"#;
        let n = Numberings::from_xml(xml.as_bytes()).unwrap();
        let mut nums = Numberings::new();
        nums = nums
            .add_abstract_numbering(
                AbstractNumbering::new(0).add_level(
                    Level::new(
                        0,
                        Start::new(1),
                        NumberFormat::new("bullet"),
                        LevelText::new("●"),
                        LevelJc::new("left"),
                    )
                    .indent(
                        Some(720),
                        Some(SpecialIndentType::Hanging(360)),
                        None,
                        None,
                    ),
                ),
            )
            .add_numbering(Numbering::new(1, 0));
        assert_eq!(n, nums)
    }

    #[test]
    fn test_numberings_from_xml_with_num_style_link() {
        let xml = r#"<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" >
    <w:abstractNum w:abstractNumId="0">
        <w:multiLevelType w:val="hybridMultilevel"/>
        <w:numStyleLink w:val="style1"/>
    </w:abstractNum>
    <w:num w:numId="1">
        <w:abstractNumId w:val="0"></w:abstractNumId>
    </w:num>
</w:numbering>"#;
        let n = Numberings::from_xml(xml.as_bytes()).unwrap();
        let mut nums = Numberings::new();
        nums = nums
            .add_abstract_numbering(AbstractNumbering::new(0).num_style_link("style1"))
            .add_numbering(Numbering::new(1, 0));
        assert_eq!(n, nums)
    }

    #[test]
    fn test_numberings_from_xml_with_style_link() {
        let xml = r#"<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" >
    <w:abstractNum w:abstractNumId="0">
        <w:multiLevelType w:val="hybridMultilevel"/>
        <w:styleLink w:val="style1"/>
    </w:abstractNum>
    <w:num w:numId="1">
        <w:abstractNumId w:val="0"></w:abstractNumId>
    </w:num>
</w:numbering>"#;
        let n = Numberings::from_xml(xml.as_bytes()).unwrap();
        let mut nums = Numberings::new();
        nums = nums
            .add_abstract_numbering(AbstractNumbering::new(0).style_link("style1"))
            .add_numbering(Numbering::new(1, 0));
        assert_eq!(n, nums)
    }

    #[test]
    fn test_numberings_from_xml_with_override() {
        let xml = r#"<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" >
    <w:abstractNum w:abstractNumId="0">
        <w:multiLevelType w:val="hybridMultilevel"/>
    </w:abstractNum>
    <w:num w:numId="1">
        <w:abstractNumId w:val="0"></w:abstractNumId>
        <w:lvlOverride w:ilvl="0">
          <w:startOverride w:val="1"/>
        </w:lvlOverride>
        <w:lvlOverride w:ilvl="1">
          <w:startOverride w:val="1"/>
        </w:lvlOverride>
    </w:num>
</w:numbering>"#;
        let n = Numberings::from_xml(xml.as_bytes()).unwrap();
        let mut nums = Numberings::new();
        let overrides = vec![
            LevelOverride::new(0).start(1),
            LevelOverride::new(1).start(1),
        ];
        let num = Numbering::new(1, 0).overrides(overrides);
        nums = nums
            .add_abstract_numbering(AbstractNumbering::new(0))
            .add_numbering(num);
        assert_eq!(n, nums)
    }
}
