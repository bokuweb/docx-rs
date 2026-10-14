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
                        XMLElement::KeepNext => {
                            p.keep_next = Some(read_bool(&attributes));
                        }
                        XMLElement::KeepLines => {
                            p.keep_lines = Some(read_bool(&attributes));
                        }
                        XMLElement::PageBreakBefore => {
                            p.page_break_before = Some(read_bool(&attributes));
                        }
                        XMLElement::WidowControl => {
                            p.widow_control = Some(read_bool(&attributes));
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

    fn parse_properties(body: &str) -> ParagraphProperty {
        let xml = format!(
            r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">{body}</w:pPr>"#
        );
        ParagraphProperty::read(&mut EventReader::new(xml.as_bytes()), &[]).unwrap()
    }

    fn flags(property: &ParagraphProperty) -> [Option<bool>; 4] {
        [
            property.keep_next,
            property.keep_lines,
            property.page_break_before,
            property.widow_control,
        ]
    }

    #[test]
    fn pagination_flags_omitted_remain_unset() {
        assert_eq!(flags(&parse_properties("")), [None; 4]);
    }

    #[test]
    fn pagination_flags_true_forms_are_retained() {
        for attribute in ["", r#" w:val="1""#, r#" w:val="true""#, r#" w:val="on""#] {
            let body = ["keepNext", "keepLines", "pageBreakBefore", "widowControl"]
                .map(|element| format!("<w:{element}{attribute}/>"))
                .join("");
            assert_eq!(flags(&parse_properties(&body)), [Some(true); 4], "{body}");
        }
    }

    macro_rules! false_pagination_flag_test {
        ($test:ident, $element:literal, $field:ident) => {
            #[test]
            fn $test() {
                for value in ["0", "false", "off"] {
                    let body = format!("<w:{} w:val=\"{value}\"/>", $element);
                    assert_eq!(parse_properties(&body).$field, Some(false), "{body}");
                }
            }
        };
    }

    false_pagination_flag_test!(pagination_flags_keep_next_false, "keepNext", keep_next);
    false_pagination_flag_test!(pagination_flags_keep_lines_false, "keepLines", keep_lines);
    false_pagination_flag_test!(
        pagination_flags_page_break_before_false,
        "pageBreakBefore",
        page_break_before
    );
    false_pagination_flag_test!(
        pagination_flags_widow_control_false,
        "widowControl",
        widow_control
    );

    #[test]
    fn pagination_flags_later_values_replace_earlier_values() {
        for (first, last, expected) in [("1", "0", false), ("0", "1", true)] {
            let body = ["keepNext", "keepLines", "pageBreakBefore", "widowControl"]
                .map(|element| {
                    format!("<w:{element} w:val=\"{first}\"/><w:{element} w:val=\"{last}\"/>")
                })
                .join("");
            assert_eq!(
                flags(&parse_properties(&body)),
                [Some(expected); 4],
                "{body}"
            );
        }
    }

    #[test]
    fn pagination_flags_false_survives_a_named_style() {
        let xml = r#"<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
          <w:style w:type="paragraph" w:styleId="ClearPagination">
            <w:pPr><w:keepNext w:val="0"/><w:keepLines w:val="false"/>
            <w:pageBreakBefore w:val="off"/><w:widowControl w:val="0"/></w:pPr>
          </w:style>
        </w:styles>"#;
        let styles = Styles::from_xml(xml.as_bytes()).unwrap();
        assert_eq!(styles.styles.len(), 1);
        assert_eq!(
            flags(&styles.styles[0].paragraph_property),
            [Some(false); 4]
        );
    }
}
