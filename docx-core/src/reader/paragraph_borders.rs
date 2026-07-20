use std::io::Read;
use std::str::FromStr;

use super::*;
use crate::types::*;

impl ElementReader for ParagraphBorders {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut borders = ParagraphBorders::with_empty();
        loop {
            match r.next_event() {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let position = match XMLElement::from_str(&name.local_name).unwrap() {
                        XMLElement::Top => Some(ParagraphBorderPosition::Top),
                        XMLElement::Left => Some(ParagraphBorderPosition::Left),
                        XMLElement::Bottom => Some(ParagraphBorderPosition::Bottom),
                        XMLElement::Right => Some(ParagraphBorderPosition::Right),
                        XMLElement::Between => Some(ParagraphBorderPosition::Between),
                        XMLElement::Bar => Some(ParagraphBorderPosition::Bar),
                        _ => None,
                    };

                    if let Some(position) = position {
                        let attributes = read_border(&attributes)?;
                        let mut border = ParagraphBorder::new(position)
                            .val(attributes.border_type)
                            .color(attributes.color);
                        if let Some(size) = attributes.size {
                            border = border.size(size as usize);
                        }
                        if let Some(space) = attributes.space {
                            border = border.space(space as usize);
                        }
                        borders = borders.set(border);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if XMLElement::from_str(&name.local_name).unwrap()
                        == XMLElement::ParagraphBorders
                    {
                        return Ok(borders);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
