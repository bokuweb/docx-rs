use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

use super::attributes::*;
use crate::types::*;

impl ElementReader for ParagraphProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut p = ParagraphProperty::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Indent => {
                            let (start, end, special, start_chars, hanging_chars, first_line_chars) =
                                read_indent(&attributes)?;
                            p = p.indent(start, special, end, start_chars);

                            if let Some(chars) = hanging_chars {
                                p = p.hanging_chars(chars);
                            }
                            if let Some(chars) = first_line_chars {
                                p = p.first_line_chars(chars);
                            }
                            continue;
                        }
                        XMLElement::Spacing => {
                            if let Ok(spacing) =
                                attributes::line_spacing::read_line_spacing(&attributes)
                            {
                                p = p.line_spacing(spacing);
                            }
                            continue;
                        }
                        XMLElement::Justification => {
                            if let Ok(v) = AlignmentType::from_str(&attributes[0].value) {
                                p = p.align(v);
                            }
                            continue;
                        }
                        XMLElement::ParagraphStyle => {
                            p = p.style(&attributes[0].value);
                            continue;
                        }
                        XMLElement::RunProperty => {
                            if let Ok(run_pr) = RunProperty::read(r, attrs) {
                                p.run_property = run_pr;
                            }
                            continue;
                        }
                        XMLElement::DivId => {
                            if let Some(val) = read_val(&attributes) {
                                p.div_id = Some(val)
                            }
                            continue;
                        }
                        XMLElement::NumberingProperty => {
                            if let Ok(num_pr) = NumberingProperty::read(r, attrs) {
                                p = p.numbering_property(num_pr);
                            }
                            continue;
                        }
                        XMLElement::OutlineLvl => {
                            if let Some(val) = read_val(&attributes) {
                                if let Ok(val) = usize::from_str(&val) {
                                    p = p.outline_lvl(val);
                                }
                            }
                            continue;
                        }
                        XMLElement::KeepNext => {
                            p.keep_next = Some(true);
                        }
                        XMLElement::KeepLines => {
                            p.keep_lines = Some(true);
                        }
                        XMLElement::PageBreakBefore => {
                            p.page_break_before = Some(true);
                        }
                        XMLElement::WidowControl => {
                            p.widow_control = Some(true);
                        }
                        XMLElement::ParagraphPropertyChange => {
                            if let Ok(ppr_change) = ParagraphPropertyChange::read(r, &attributes) {
                                p.paragraph_property_change = Some(ppr_change);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::ParagraphProperty {
                        return Ok(p);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
