use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

use super::attributes::*;
use crate::types::*;

fn read_lineheight(attributes: &[OwnedAttribute]) -> Option<u32> {
    for a in attributes {
        let local_name = &a.name.local_name;
        if let "line" = local_name.as_str() {
            return value_to_dax(&a.value).ok().map(|l| l as u32);
        }
    }
    None
}

impl ElementReader for Paragraph {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut p = Paragraph::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    ignore::ignore_element(e.clone(), XMLElement::ParagraphPropertyChange, r);

                    match e {
                        XMLElement::Run => {
                            let run = Run::read(r, attrs)?;
                            p = p.add_run(run);
                            continue;
                        }
                        XMLElement::Insert => {
                            let ins = Insert::read(r, &attributes)?;
                            p = p.add_insert(ins);
                            continue;
                        }
                        XMLElement::Delete => {
                            let del = Delete::read(r, &attributes)?;
                            p = p.add_delete(del);
                            continue;
                        }
                        XMLElement::BookmarkStart => {
                            let s = BookmarkStart::read(r, &attributes)?;
                            p = p.add_bookmark_start(s.id, s.name);
                            continue;
                        }
                        XMLElement::BookmarkEnd => {
                            let e = BookmarkEnd::read(r, &attributes)?;
                            p = p.add_bookmark_end(e.id);
                            continue;
                        }
                        XMLElement::CommentRangeStart => {
                            // TODO: Support comment later.
                            continue;
                        }
                        XMLElement::CommentRangeEnd => {
                            p = p.add_comment_end(usize::from_str(&attributes[0].value)?);
                            continue;
                        }
                        XMLElement::Indent => {
                            let (start, end, special, start_chars) = read_indent(&attributes)?;
                            p = p.indent(start, special, end, start_chars);
                            continue;
                        }
                        XMLElement::Spacing => {
                            if let Some(line) = read_lineheight(&attributes) {
                                p = p.line_height(line);
                            }
                            continue;
                        }
                        XMLElement::Justification => {
                            p = p.align(AlignmentType::from_str(&attributes[0].value)?);
                            continue;
                        }
                        XMLElement::ParagraphStyle => {
                            p = p.style(&attributes[0].value);
                            continue;
                        }
                        XMLElement::NumberingProperty => {
                            let num_pr = NumberingProperty::read(r, attrs)?;
                            if num_pr.id.is_some() && num_pr.level.is_some() {
                                p = p.numbering(num_pr.id.unwrap(), num_pr.level.unwrap());
                            }
                            continue;
                        }
                        XMLElement::RunProperty => {
                            let run_pr = RunProperty::read(r, attrs)?;
                            p = p.run_property(run_pr);
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Paragraph {
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
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_indent() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:p>
        <w:pPr>
            <w:ind w:left="1470" w:right="1270" w:hanging="0"/>
            <w:rPr></w:rPr>
        </w:pPr>
        <w:r>
            <w:rPr></w:rPr>
            <w:t>a</w:t>
        </w:r>
    </w:p>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let p = Paragraph::read(&mut parser, &[]).unwrap();

        assert_eq!(
            p,
            Paragraph {
                id: "12345678".to_owned(),
                children: vec![ParagraphChild::Run(Box::new(Run::new().add_text("a")))],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: None,
                    numbering_property: None,
                    alignment: None,
                    indent: Some(Indent::new(
                        Some(1470),
                        Some(SpecialIndentType::Hanging(0)),
                        Some(1270),
                        None,
                    )),
                    line_height: None,
                },
                has_numbering: false,
            }
        );
    }

    #[test]
    fn test_read_indent_start_chars() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:p>
        <w:pPr>
            <w:ind w:startChars="100" />
            <w:rPr></w:rPr>
        </w:pPr>
        <w:r>
            <w:rPr></w:rPr>
            <w:t>a</w:t>
        </w:r>
    </w:p>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let p = Paragraph::read(&mut parser, &[]).unwrap();

        assert_eq!(
            p,
            Paragraph {
                id: "12345678".to_owned(),
                children: vec![ParagraphChild::Run(Box::new(Run::new().add_text("a")))],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: None,
                    numbering_property: None,
                    alignment: None,
                    indent: Some(Indent::new(None, None, None, Some(100))),
                    line_height: None,
                },
                has_numbering: false,
            }
        );
    }

    #[test]
    fn test_read_jc() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:p>
        <w:pPr>
            <w:jc w:val="left"/>
        </w:pPr>
    </w:p>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let p = Paragraph::read(&mut parser, &[]).unwrap();

        assert_eq!(
            p,
            Paragraph {
                id: "12345678".to_owned(),
                children: vec![],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: None,
                    numbering_property: None,
                    alignment: Some(Justification::new(AlignmentType::Left.to_string())),
                    indent: None,
                    line_height: None,
                },
                has_numbering: false,
            }
        );
    }

    #[test]
    fn test_read_numbering() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:p>
        <w:pPr>
            <w:numPr>
                <w:ilvl w:val="0"/>
                <w:numId w:val="1"/>
            </w:numPr>
        </w:pPr>
    </w:p>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let p = Paragraph::read(&mut parser, &[]).unwrap();
        assert_eq!(
            p,
            Paragraph {
                id: "12345678".to_owned(),
                children: vec![],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: None,
                    numbering_property: Some(
                        NumberingProperty::new().add_num(NumberingId::new(1), IndentLevel::new(0),)
                    ),
                    alignment: None,
                    indent: None,
                    line_height: None,
                },
                has_numbering: true,
            }
        );
    }

    #[test]
    fn test_read_insert() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:p>
        <w:ins w:id="0" w:author="unknown" w:date="2019-11-15T14:19:04Z">
            <w:r>
                <w:rPr></w:rPr>
                <w:t>W</w:t>
            </w:r>
        </w:ins>
    </w:p>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let p = Paragraph::read(&mut parser, &[]).unwrap();
        assert_eq!(
            p,
            Paragraph {
                id: "12345678".to_owned(),
                children: vec![ParagraphChild::Insert(
                    Insert::new(Run::new().add_text("W"))
                        .author("unknown")
                        .date("2019-11-15T14:19:04Z")
                )],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: None,
                    numbering_property: None,
                    alignment: None,
                    indent: None,
                    line_height: None,
                },
                has_numbering: false,
            }
        );
    }

    #[test]
    fn test_read_delete() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:p>
        <w:del w:id="3" w:author="unknown" w:date="2019-11-15T14:19:04Z">
            <w:r>
                <w:rPr></w:rPr>
                <w:delText xml:space="preserve">Hello </w:delText>
            </w:r>
        </w:del>
    </w:p>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let p = Paragraph::read(&mut parser, &[]).unwrap();

        assert_eq!(
            p,
            Paragraph {
                id: "12345678".to_owned(),
                children: vec![ParagraphChild::Delete(
                    Delete::new(Run::new().add_delete_text("Hello "))
                        .author("unknown")
                        .date("2019-11-15T14:19:04Z")
                )],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: None,
                    numbering_property: None,
                    alignment: None,
                    indent: None,
                    line_height: None,
                },
                has_numbering: false,
            }
        );
    }

    #[test]
    fn test_read_bookmark() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:p>
        <w:bookmarkStart w:id="0" w:name="ABCD-1234"/>
            <w:r>
                <w:rPr></w:rPr>
                <w:t>Bookmarked</w:t>
            </w:r>
        <w:bookmarkEnd w:id="0"/>
    </w:p>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let p = Paragraph::read(&mut parser, &[]).unwrap();

        assert_eq!(
            p,
            Paragraph {
                id: "12345678".to_owned(),
                children: vec![
                    ParagraphChild::BookmarkStart(BookmarkStart::new(0, "ABCD-1234")),
                    ParagraphChild::Run(Box::new(Run::new().add_text("Bookmarked"))),
                    ParagraphChild::BookmarkEnd(BookmarkEnd::new(0)),
                ],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: None,
                    numbering_property: None,
                    alignment: None,
                    indent: None,
                    line_height: None,
                },
                has_numbering: false,
            }
        );
    }

    #[test]
    fn test_read_two_insert() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:p>
        <w:ins w:id="0" w:author="unknown" w:date="2019-11-15T14:19:04Z">
            <w:r>
                <w:rPr></w:rPr>
                <w:t>W</w:t>
            </w:r>
        </w:ins>
        <w:ins w:id="0" w:author="unknown" w:date="2019-11-15T14:19:04Z">
            <w:r>
                <w:rPr></w:rPr>
                <w:t>H</w:t>
            </w:r>
        </w:ins>
    </w:p>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let p = Paragraph::read(&mut parser, &[]).unwrap();
        assert_eq!(
            p,
            Paragraph {
                id: "12345678".to_owned(),
                children: vec![
                    ParagraphChild::Insert(
                        Insert::new(Run::new().add_text("W"))
                            .author("unknown")
                            .date("2019-11-15T14:19:04Z")
                    ),
                    ParagraphChild::Insert(
                        Insert::new(Run::new().add_text("H"))
                            .author("unknown")
                            .date("2019-11-15T14:19:04Z")
                    )
                ],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: None,
                    numbering_property: None,
                    alignment: None,
                    indent: None,
                    line_height: None,
                },
                has_numbering: false,
            }
        );
    }

    #[test]
    fn test_read_two_run_in_insert() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:p>
        <w:ins w:id="0" w:author="unknown" w:date="2019-11-15T14:19:04Z">
            <w:r>
                <w:rPr></w:rPr>
                <w:t>W</w:t>
            </w:r>
            <w:r>
                <w:rPr></w:rPr>
                <w:t>H</w:t>
            </w:r>
        </w:ins>
    </w:p>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let p = Paragraph::read(&mut parser, &[]).unwrap();
        assert_eq!(
            p,
            Paragraph {
                id: "12345678".to_owned(),
                children: vec![ParagraphChild::Insert(
                    Insert::new(Run::new().add_text("W"))
                        .author("unknown")
                        .date("2019-11-15T14:19:04Z")
                        .add_run(Run::new().add_text("H")),
                )],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: None,
                    numbering_property: None,
                    alignment: None,
                    indent: None,
                    line_height: None,
                },
                has_numbering: false,
            }
        );
    }
}
