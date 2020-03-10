use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

use super::attributes::*;
use crate::types::*;

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
                            let mut id: Option<usize> = None;
                            let mut name: Option<String> = None;

                            for a in attributes {
                                let local_name = &a.name.local_name;
                                if local_name == "id" {
                                    id = Some(usize::from_str(&a.value)?);
                                } else if local_name == "name" {
                                    name = Some(a.value.clone());
                                }
                            }
                            if id.is_none() || name.is_none() {
                                return Err(ReaderError::XMLReadError);
                            }
                            p = p.add_bookmark_start(id.unwrap(), name.unwrap());
                            continue;
                        }
                        XMLElement::BookmarkEnd => {
                            let mut id: Option<usize> = None;
                            for a in attributes {
                                let local_name = &a.name.local_name;
                                if local_name == "id" {
                                    id = Some(usize::from_str(&a.value)?);
                                }
                            }
                            if let Some(id) = id {
                                p = p.add_bookmark_end(id);
                            } else {
                                return Err(ReaderError::XMLReadError);
                            }
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
                            p = p.numbering(num_pr.id, num_pr.level);
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
        let s: Option<&str> = None;
        assert_eq!(
            p,
            Paragraph {
                children: vec![ParagraphChild::Run(Run::new().add_text("a"))],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: ParagraphStyle::new(s),
                    numbering_property: None,
                    alignment: None,
                    indent: Some(Indent::new(
                        Some(1470),
                        Some(SpecialIndentType::Hanging(0)),
                        Some(1270),
                        None,
                    )),
                },
                has_numbering: false,
                attrs: Vec::new(),
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
        let s: Option<&str> = None;
        assert_eq!(
            p,
            Paragraph {
                children: vec![ParagraphChild::Run(Run::new().add_text("a"))],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: ParagraphStyle::new(s),
                    numbering_property: None,
                    alignment: None,
                    indent: Some(Indent::new(None, None, None, Some(100))),
                },
                has_numbering: false,
                attrs: Vec::new(),
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
        let s: Option<&str> = None;
        assert_eq!(
            p,
            Paragraph {
                children: vec![],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: ParagraphStyle::new(s),
                    numbering_property: None,
                    alignment: Some(Justification::new(AlignmentType::Left.to_string())),
                    indent: None,
                },
                has_numbering: false,
                attrs: vec![],
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
        let s: Option<&str> = None;
        assert_eq!(
            p,
            Paragraph {
                children: vec![],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: ParagraphStyle::new(s),
                    numbering_property: Some(NumberingProperty::new(
                        NumberingId::new(1),
                        IndentLevel::new(0),
                    )),
                    alignment: None,
                    indent: None,
                },
                has_numbering: true,
                attrs: vec![],
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
        let s: Option<&str> = None;
        assert_eq!(
            p,
            Paragraph {
                children: vec![ParagraphChild::Insert(
                    Insert::new(Run::new().add_text("W"))
                        .author("unknown")
                        .date("2019-11-15T14:19:04Z")
                )],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: ParagraphStyle::new(s),
                    numbering_property: None,
                    alignment: None,
                    indent: None,
                },
                has_numbering: false,
                attrs: vec![],
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
        let s: Option<&str> = None;
        assert_eq!(
            p,
            Paragraph {
                children: vec![ParagraphChild::Delete(
                    Delete::new(Run::new().add_delete_text("Hello "))
                        .author("unknown")
                        .date("2019-11-15T14:19:04Z")
                )],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: ParagraphStyle::new(s),
                    numbering_property: None,
                    alignment: None,
                    indent: None,
                },
                has_numbering: false,
                attrs: vec![],
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
        let s: Option<&str> = None;
        assert_eq!(
            p,
            Paragraph {
                children: vec![
                    ParagraphChild::BookmarkStart(BookmarkStart::new(0, "ABCD-1234")),
                    ParagraphChild::Run(Run::new().add_text("Bookmarked")),
                    ParagraphChild::BookmarkEnd(BookmarkEnd::new(0)),
                ],
                property: ParagraphProperty {
                    run_property: RunProperty::new(),
                    style: ParagraphStyle::new(s),
                    numbering_property: None,
                    alignment: None,
                    indent: None,
                },
                has_numbering: false,
                attrs: vec![],
            }
        );
    }
}
