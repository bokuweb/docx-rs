use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::VertAlignType;

use super::*;

fn read_run_fonts(attributes: &[OwnedAttribute]) -> Result<RunFonts, ReaderError> {
    let mut f = RunFonts::new();
    for a in attributes {
        let local_name = &a.name.local_name;
        match local_name.as_str() {
            "asciiTheme" => {
                f = f.ascii_theme(&a.value);
            }
            "eastAsiaTheme" => {
                f = f.east_asia_theme(&a.value);
            }
            "hAnsiTheme" => {
                f = f.hi_ansi_theme(&a.value);
            }
            "cstheme" => {
                f = f.cs_theme(&a.value);
            }
            "ascii" => {
                f = f.ascii(&a.value);
            }
            "eastAsia" => {
                f = f.east_asia(&a.value);
            }
            "hAnsi" => {
                f = f.hi_ansi(&a.value);
            }
            "cs" => {
                f = f.cs(&a.value);
            }
            "hint" => {
                f = f.hint(&a.value);
            }
            _ => {}
        }
    }
    Ok(f)
}

impl ElementReader for RunProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut rp = RunProperty::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::RunStyle => {
                            if let Some(v) = read_val(&attributes) {
                                rp = rp.style(&v);
                            }
                        }
                        XMLElement::Bold => {
                            if !read_bool(&attributes) {
                                rp = rp.disable_bold();
                                continue;
                            }
                            rp = rp.bold();
                        }
                        XMLElement::Highlight => rp = rp.highlight(attributes[0].value.clone()),
                        XMLElement::Strike => {
                            if !read_bool(&attributes) {
                                rp.strike = Some(Strike::new().disable());
                                continue;
                            }
                            rp = rp.strike();
                        }
                        XMLElement::VertAlign => {
                            if let Ok(v) = VertAlignType::from_str(&attributes[0].value) {
                                rp = rp.vert_align(v)
                            }
                        }
                        XMLElement::Color => rp = rp.color(attributes[0].value.clone()),
                        XMLElement::Size => rp = rp.size(usize::from_str(&attributes[0].value)?),
                        XMLElement::Spacing => {
                            if let Some(v) = read_val(&attributes) {
                                let v = value_to_dax(&v)?;
                                rp = rp.spacing(v)
                            }
                        }
                        XMLElement::RunFonts => {
                            if let Ok(f) = read_run_fonts(&attributes) {
                                rp = rp.fonts(f);
                            }
                        }
                        XMLElement::Underline => rp = rp.underline(&attributes[0].value.clone()),
                        XMLElement::Italic => {
                            if !read_bool(&attributes) {
                                rp = rp.disable_italic();
                                continue;
                            }
                            rp = rp.italic();
                        }
                        XMLElement::Vanish => rp = rp.vanish(),
                        XMLElement::TextBorder => {
                            if let Ok(attr) = read_border(&attributes) {
                                let mut border = TextBorder::new()
                                    .border_type(attr.border_type)
                                    .color(attr.color);
                                if let Some(size) = attr.size {
                                    border = border.size(size as usize);
                                };
                                rp = rp.text_border(border);
                                continue;
                            }
                        }
                        XMLElement::Insert => {
                            if let Ok(ins) = Insert::read(r, &attributes) {
                                rp = rp.insert(ins);
                            }
                        }
                        XMLElement::Delete => {
                            if let Ok(del) = Delete::read(r, &attributes) {
                                rp = rp.delete(del);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::RunProperty {
                        return Ok(rp);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
