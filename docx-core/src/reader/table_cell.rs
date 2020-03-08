use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::types::*;

impl ElementReader for TableCell {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut cell = TableCell::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Paragraph => {
                            let p = Paragraph::read(r, &attributes)?;
                            cell = cell.add_paragraph(p);
                            continue;
                        }
                        XMLElement::TableCellProperty => loop {
                            let e = r.next();
                            match e {
                                Ok(XmlEvent::StartElement {
                                    attributes, name, ..
                                }) => {
                                    let e = XMLElement::from_str(&name.local_name).unwrap();
                                    match e {
                                        XMLElement::TableCellWidth => {
                                            let mut w = 0;
                                            let mut width_type = WidthType::Auto;
                                            for a in attributes {
                                                let local_name = &a.name.local_name;
                                                if local_name == "type" {
                                                    width_type = WidthType::from_str(&a.value)?;
                                                } else if local_name == "w" {
                                                    w = usize::from_str(&a.value)?;
                                                }
                                            }
                                            cell = cell.width(w, width_type);
                                        }
                                        XMLElement::TableGridSpan => {
                                            if let Some(a) = &attributes.get(0) {
                                                cell = cell.grid_span(usize::from_str(&a.value)?)
                                            }
                                        }
                                        XMLElement::TableVMerge => {
                                            if let Some(a) = &attributes.get(0) {
                                                cell = cell.vertical_merge(VMergeType::from_str(
                                                    &a.value,
                                                )?);
                                            } else {
                                                // Treat as a continue without attribute
                                                cell = cell.vertical_merge(VMergeType::Continue)
                                            }
                                        }
                                        XMLElement::TableCellBorders => {
                                            // TODO: Support table cell borders later
                                        }
                                        _ => {}
                                    }
                                }
                                Ok(XmlEvent::EndElement { name, .. }) => {
                                    let e = XMLElement::from_str(&name.local_name).unwrap();
                                    if e == XMLElement::TableCellProperty {
                                        break;
                                    }
                                }
                                Err(_) => return Err(ReaderError::XMLReadError),
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableCell {
                        return Ok(cell);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_cell_with_prop() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:tc>
    <w:tcPr>
        <w:tcW w:w="6425" w:type="dxa"/>
        <w:vMerge w:val="restart"/>
        <w:gridSpan w:val="2"/>
        <w:tcBorders>
            <w:top w:val="single" w:sz="2" w:space="0" w:color="000000"/>
            <w:left w:val="single" w:sz="2" w:space="0" w:color="000000"/>
            <w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000"/>
            <w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000"/>
        </w:tcBorders>
        <w:shd w:fill="auto" w:val="clear"/>
    </w:tcPr>
    <w:p>
        <w:r>
            <w:rPr></w:rPr>
        </w:r>
    </w:p>
</w:tc>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let cell = TableCell::read(&mut parser, &[]).unwrap();
        assert_eq!(
            cell,
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new()))
                .width(6425, WidthType::DXA)
                .grid_span(2)
                .vertical_merge(VMergeType::Restart),
        );
    }

    #[test]
    fn test_read_no_attr_vmerge() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:tc>
    <w:tcPr>
        <w:tcW w:w="6425" w:type="dxa"/>
        <w:vMerge />
        <w:shd w:fill="auto" w:val="clear"/>
    </w:tcPr>
    <w:p>
        <w:r>
            <w:rPr></w:rPr>
        </w:r>
    </w:p>
</w:tc>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let cell = TableCell::read(&mut parser, &[]).unwrap();
        assert_eq!(
            cell,
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new()))
                .width(6425, WidthType::DXA)
                .vertical_merge(VMergeType::Continue),
        );
    }
}
