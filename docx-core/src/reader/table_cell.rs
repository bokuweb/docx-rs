use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

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
                    // FIXME: ignore table in table for now. Please support table in table later.
                    ignore::ignore_element(e.clone(), XMLElement::Table, r);
                    match e {
                        XMLElement::Paragraph => {
                            let p = Paragraph::read(r, &attributes)?;
                            cell = cell.add_paragraph(p);
                            continue;
                        }
                        XMLElement::TableCellProperty => {
                            if let Ok(p) = TableCellProperty::read(r, &attributes) {
                                cell.property = p;
                            }
                            continue;
                        }
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
    use crate::types::*;
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
            <w:left w:val="single" w:sz="3" w:space="0" w:color="000000"/>
            <w:right w:val="single" w:sz="2" w:space="0" w:color="000000"/>
            <w:bottom w:val="double" w:sz="4" w:space="0" w:color="000000"/>
            <w:insideH w:val="single" w:sz="5" w:space="0" w:color="FF0000"/>
            <w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000"/>
        </w:tcBorders>
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
                .vertical_merge(VMergeType::Restart)
                .set_border(TableCellBorder::new(TableCellBorderPosition::Top))
                .set_border(TableCellBorder::new(TableCellBorderPosition::Left).size(3))
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Bottom)
                        .size(4)
                        .border_type(BorderType::Double)
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::InsideH)
                        .size(5)
                        .color("FF0000".to_owned())
                )
        );
    }

    #[test]
    fn test_read_no_attr_vmerge() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:tc>
    <w:tcPr>
        <w:tcW w:w="6425" w:type="dxa"/>
        <w:vMerge />
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

    #[test]
    fn test_read_valign() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:tc>
    <w:tcPr>
        <w:tcW w:w="6425" w:type="dxa"/>
        <w:vAlign w:val="bottom"/>
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
                .vertical_align(VAlignType::Bottom),
        );
    }
}
