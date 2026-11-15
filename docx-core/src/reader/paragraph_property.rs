use std::io::Read;
use std::str::FromStr;

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
            let e = r.next_event();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Indent => {
                            let (
                                start,
                                end,
                                special,
                                start_chars,
                                end_chars,
                                hanging_chars,
                                first_line_chars,
                            ) = read_indent(&attributes)?;
                            p = p.indent(start, special, end, start_chars);

                            if let Some(chars) = end_chars {
                                p = p.end_chars(chars);
                            }

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
                        XMLElement::TextAlignment => {
                            if let Ok(v) = TextAlignmentType::from_str(&attributes[0].value) {
                                p = p.text_alignment(v);
                            }
                            continue;
                        }
                        XMLElement::AdjustRightInd => {
                            if let Some(val) = read_val(&attributes) {
                                if let Ok(v) = isize::from_str(&val) {
                                    p = p.adjust_right_ind(v);
                                }
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
                        XMLElement::SnapToGrid => {
                            let v = read_bool(&attributes);
                            p.snap_to_grid = Some(v);
                        }
                        XMLElement::ContextualSpacing => {
                            p.contextual_spacing = Some(read_bool(&attributes));
                        }
                        XMLElement::Kinsoku => {
                            p.kinsoku = Some(read_bool(&attributes));
                        }
                        XMLElement::WordWrap => {
                            p.word_wrap = Some(read_bool(&attributes));
                        }
                        XMLElement::OverflowPunct => {
                            p.overflow_punct = Some(read_bool(&attributes));
                        }
                        XMLElement::TopLinePunct => {
                            p.top_line_punct = Some(read_bool(&attributes));
                        }
                        XMLElement::AutoSpaceDN => {
                            p.auto_space_dn = Some(read_bool(&attributes));
                        }
                        XMLElement::AutoSpaceDE => {
                            p.auto_space_de = Some(read_bool(&attributes));
                        }
                        XMLElement::KeepNext => {
                            if read_bool(&attributes) {
                                p.keep_next = Some(true);
                            }
                        }
                        XMLElement::KeepLines => {
                            if read_bool(&attributes) {
                                p.keep_lines = Some(true);
                            }
                        }
                        XMLElement::PageBreakBefore => {
                            if read_bool(&attributes) {
                                p.page_break_before = Some(true);
                            }
                        }
                        XMLElement::WidowControl => {
                            if read_bool(&attributes) {
                                p.widow_control = Some(true);
                            }
                        }
                        XMLElement::ParagraphPropertyChange => {
                            if let Ok(ppr_change) = ParagraphPropertyChange::read(r, &attributes) {
                                p.paragraph_property_change = Some(ppr_change);
                            }
                        }
                        XMLElement::SectionProperty => {
                            if let Ok(sp) = SectionProperty::read(r, &attributes) {
                                p.section_property = Some(sp);
                            }
                        }
                        XMLElement::FrameProperty => {
                            if let Ok(pr) = FrameProperty::read(r, &attributes) {
                                p.frame_property = Some(pr);
                            }
                        }
                        XMLElement::Shading => {
                            if let Ok(shd) = Shading::read(r, &attributes) {
                                p.shading = Some(shd);
                            }
                        }
                        XMLElement::ParagraphBorders => {
                            if let Ok(borders) = ParagraphBorders::read(r, &attributes) {
                                p = p.set_borders(borders);
                            }
                        }
                        XMLElement::Tabs => {
                            if let Ok(tabs) = Tabs::read(r, &attributes) {
                                for t in tabs.tabs {
                                    p = p.add_tab(t);
                                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::documents::BuildXML;

    #[test]
    fn reads_and_writes_east_asian_typography() {
        let xml = r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:kinsoku/><w:wordWrap w:val="0"/>
            <w:overflowPunct w:val="false"/><w:topLinePunct w:val="on"/>
            <w:autoSpaceDE/><w:autoSpaceDN w:val="off"/><w:textAlignment w:val="center"/></w:pPr>"#;
        let property = ParagraphProperty::read(&mut EventReader::new(xml.as_bytes()), &[]).unwrap();
        assert_eq!(property.auto_space_de, Some(true));
        assert_eq!(property.auto_space_dn, Some(false));

        let json = serde_json::to_value(&property).unwrap();
        assert_eq!(json["kinsoku"], true);
        assert_eq!(json["wordWrap"], false);
        assert_eq!(json["overflowPunct"], false);
        assert_eq!(json["topLinePunct"], true);
        assert_eq!(json["textAlignment"], "center");
        assert_eq!(json["autoSpaceDE"], true);
        assert_eq!(json["autoSpaceDN"], false);

        let output = String::from_utf8(property.build()).unwrap();
        assert!(output.contains(r#"<w:kinsoku w:val="1" />"#));
        assert!(output.contains(r#"<w:wordWrap w:val="0" />"#));
        assert!(output.contains(r#"<w:overflowPunct w:val="0" />"#));
        assert!(output.contains(r#"<w:topLinePunct w:val="1" />"#));
        assert!(output.contains(r#"<w:textAlignment w:val="center" />"#));
        assert!(output.contains(r#"<w:autoSpaceDE w:val="1" />"#));
        assert!(output.contains(r#"<w:autoSpaceDN w:val="0" />"#));

        let default = ParagraphProperty::new();
        let json = serde_json::to_value(&default).unwrap();
        let output = String::from_utf8(default.build()).unwrap();
        for key in [
            "kinsoku",
            "wordWrap",
            "overflowPunct",
            "topLinePunct",
            "autoSpaceDE",
            "autoSpaceDN",
            "textAlignment",
        ] {
            assert!(json.get(key).is_none());
            assert!(!output.contains(key));
        }
    }
}
