use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for AGraphic {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut graphic = AGraphic::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = AXMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    if let AXMLElement::GraphicData = e {
                        let data = AGraphicData::read(r, &attributes)?;
                        graphic = graphic.add_graphic_data(data);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = AXMLElement::from_str(&name.local_name).unwrap();
                    if e == AXMLElement::Graphic {
                        return Ok(graphic);
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
    fn test_read_graphic_with_textbox() {
        let c = r#"<w:document xmlns:o="urn:schemas-microsoft-com:office:office"
        xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
        xmlns:v="urn:schemas-microsoft-com:vml"
        xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
        xmlns:w10="urn:schemas-microsoft-com:office:word"
        xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"
        xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"
        xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup"
        xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
        xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing"
        xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14 wp14">
        <a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape">
            <wps:wsp>
                <wps:spPr>
                    <a:xfrm>
                        <a:off x="0" y="0"/>
                        <a:ext cx="914400" cy="343080"/>
                    </a:xfrm>
                    <a:prstGeom prst="rect">
                        <a:avLst></a:avLst>
                    </a:prstGeom>
                    <a:solidFill>
                        <a:srgbClr val="ffffff"/>
                    </a:solidFill>
                    <a:ln w="720">
                        <a:solidFill>
                            <a:srgbClr val="000000"/>
                        </a:solidFill>
                        <a:round/>
                    </a:ln>
                </wps:spPr>
                <wps:style>
                    <a:lnRef idx="0"/>
                    <a:fillRef idx="0"/>
                    <a:effectRef idx="0"/>
                    <a:fontRef idx="minor"/>
                </wps:style>
                <wps:txbx>
                    <w:txbxContent>
                        <w:p>
                            <w:pPr>
                                <w:rPr></w:rPr>
                            </w:pPr>
                            <w:r>
                                <w:rPr></w:rPr>
                                <w:t>pattern1</w:t>
                            </w:r>
                        </w:p>
                    </w:txbxContent>
                </wps:txbx>
                <wps:bodyPr>
                </wps:bodyPr>
            </wps:wsp>
        </a:graphicData>
    </a:graphic></w:body>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let g = AGraphic::read(&mut parser, &[]).unwrap();
        assert_eq!(
            g,
            AGraphic::new().add_graphic_data(
                AGraphicData::new(GraphicDataType::WpShape).add_shape(
                    WpsShape::new().add_text_box(WpsTextBox::new().add_content(
                        TextBoxContent::new().add_paragraph(
                            Paragraph::new().add_run(Run::new().add_text("pattern1"))
                        )
                    ))
                )
            )
        );
    }
}
