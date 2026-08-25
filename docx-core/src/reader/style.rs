use std::io::Read;
use std::str::FromStr;

use super::*;

use crate::types::*;

impl ElementReader for Style {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut id = "".to_owned();
        let mut style_type = StyleType::Paragraph;
        let mut is_default = false;
        for a in attrs {
            let local_name = &a.name.local_name;
            if local_name == "styleId" {
                id = a.value.clone();
            } else if local_name == "type" {
                style_type = StyleType::from_str(&a.value)?;
            } else if local_name == "default" {
                is_default = !is_false(&a.value);
            }
        }
        let mut style = Style::new(id, style_type);
        style.is_default = is_default;
        loop {
            let e = r.next_event();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Name => {
                            style = style.name(attributes[0].value.clone());
                            continue;
                        }
                        XMLElement::BasedOn => {
                            if let Some(v) = read_val(&attributes) {
                                style = style.based_on(v);
                            }
                            continue;
                        }
                        XMLElement::Link => {
                            if let Some(v) = read_val(&attributes) {
                                style = style.link(v);
                            }
                            continue;
                        }
                        // pPr
                        XMLElement::ParagraphProperty => {
                            if let Ok(pr) = ParagraphProperty::read(r, attrs) {
                                style.paragraph_property = pr;
                            }
                            continue;
                        }
                        // rPr
                        XMLElement::RunProperty => {
                            let p = RunProperty::read(r, &attributes)?;
                            style.run_property = p;
                        }
                        XMLElement::TableProperty => {
                            if let Ok(p) = TableProperty::read(r, &attributes) {
                                style = style.table_property(p);
                            }
                        }
                        XMLElement::TableCellProperty => {
                            if let Ok(p) = TableCellProperty::read(r, &attributes) {
                                style = style.table_cell_property(p);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Style = e {
                        return Ok(style);
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

    #[test]
    fn reads_default_style_attribute() {
        let xml = r#"<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:style w:type="table" w:default="1" w:styleId="TableNormal"><w:name w:val="Normal Table"/></w:style></w:styles>"#;
        let styles = Styles::from_xml(xml.as_bytes()).unwrap();
        let style = &styles.styles[0];

        assert!(style.is_default);
        assert_eq!(style.style_id, "TableNormal");
    }
}
