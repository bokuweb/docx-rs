use std::collections::VecDeque;
use std::io::Read;
use std::marker::PhantomData;

use quick_xml::encoding::Decoder;
use quick_xml::escape::unescape;
use quick_xml::events::{BytesEnd, BytesRef, BytesStart, BytesText, Event};
use quick_xml::name::{NamespaceResolver, ResolveResult};
use quick_xml::reader::NsReader;
use quick_xml::XmlVersion;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedName {
    pub local_name: String,
    pub namespace: Option<String>,
    pub prefix: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedAttribute {
    pub name: OwnedName,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Namespace {
    mappings: Vec<(String, String)>,
}

impl Namespace {
    pub fn empty() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }
}

impl IntoIterator for Namespace {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<(String, String)>;

    fn into_iter(self) -> Self::IntoIter {
        self.mappings.into_iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum XmlEvent {
    StartElement {
        name: OwnedName,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    },
    EndElement {
        name: OwnedName,
    },
    Characters(String),
    Whitespace(String),
    EndDocument,
}

pub struct EventReader<R: Read> {
    // Parses `_data` without copying each event into a buffer. Declared before
    // `_data` so it is dropped first; the boxed bytes never move or change.
    reader: NsReader<&'static [u8]>,
    _data: Box<[u8]>,
    read_error: Option<std::io::Error>,
    pending: VecDeque<XmlEvent>,
    finished: bool,
    source: PhantomData<R>,
}

impl<R: Read> EventReader<R> {
    pub fn new(mut source: R) -> Self {
        let mut data = Vec::new();
        let read_error = source.read_to_end(&mut data).err();
        let data = data.into_boxed_slice();
        // SAFETY: `data` is owned by this reader, is never mutated, and outlives
        // `reader` (see the field order). Events are converted to owned values
        // before they leave this module.
        let bytes: &'static [u8] = unsafe { std::slice::from_raw_parts(data.as_ptr(), data.len()) };
        let mut reader = NsReader::from_reader(bytes);
        {
            let config = reader.config_mut();
            config.trim_text(false);
            config.check_end_names = true;
            config.expand_empty_elements = false;
        }
        Self {
            reader,
            _data: data,
            read_error,
            pending: VecDeque::new(),
            finished: false,
            source: PhantomData,
        }
    }

    pub fn next_event(&mut self) -> Result<XmlEvent, quick_xml::Error> {
        self.read_next()
    }

    fn read_next(&mut self) -> Result<XmlEvent, quick_xml::Error> {
        if let Some(event) = self.pending.pop_front() {
            return Ok(event);
        }

        if let Some(error) = self.read_error.take() {
            return Err(quick_xml::Error::Io(error.into()));
        }
        loop {
            match self.reader.read_event()? {
                Event::Start(element) => {
                    let decoder = self.reader.decoder();
                    let event = Self::build_start_event(element, decoder, self.reader.resolver())?;
                    return Ok(event);
                }
                Event::Empty(element) => {
                    let decoder = self.reader.decoder();
                    let event = Self::build_start_event(element, decoder, self.reader.resolver())?;
                    if let XmlEvent::StartElement { name, .. } = &event {
                        self.pending
                            .push_back(XmlEvent::EndElement { name: name.clone() });
                    }
                    return Ok(event);
                }
                Event::End(element) => {
                    let name = build_name_from_end(&element, self.reader.resolver())?;
                    return Ok(XmlEvent::EndElement { name });
                }
                Event::Text(text) => {
                    let text = decode_text(text)?;
                    return self.read_text_event(text);
                }
                Event::GeneralRef(reference) => {
                    let text = decode_reference(reference)?;
                    return self.read_text_event(text);
                }
                Event::CData(text) => {
                    let decoded = self.reader.decoder().decode(text.as_ref())?.into_owned();
                    return Ok(XmlEvent::Characters(decoded));
                }
                Event::Eof => {
                    self.finished = true;
                    return Ok(XmlEvent::EndDocument);
                }
                Event::Decl(_) | Event::PI(_) | Event::Comment(_) | Event::DocType(_) => {
                    // Skip non-structural events
                }
            }
        }
    }

    fn read_text_event(&mut self, mut text: String) -> Result<XmlEvent, quick_xml::Error> {
        loop {
            match self.reader.read_event()? {
                Event::Text(next_text) => {
                    text.push_str(&decode_text(next_text)?);
                }
                Event::GeneralRef(reference) => {
                    text.push_str(&decode_reference(reference)?);
                }
                Event::Start(element) => {
                    let decoder = self.reader.decoder();
                    let event = Self::build_start_event(element, decoder, self.reader.resolver())?;
                    self.pending.push_back(event);
                    break;
                }
                Event::Empty(element) => {
                    let decoder = self.reader.decoder();
                    let event = Self::build_start_event(element, decoder, self.reader.resolver())?;
                    let end_event = if let XmlEvent::StartElement { name, .. } = &event {
                        Some(XmlEvent::EndElement { name: name.clone() })
                    } else {
                        None
                    };
                    self.pending.push_back(event);
                    if let Some(end_event) = end_event {
                        self.pending.push_back(end_event);
                    }
                    break;
                }
                Event::End(element) => {
                    let name = build_name_from_end(&element, self.reader.resolver())?;
                    self.pending.push_back(XmlEvent::EndElement { name });
                    break;
                }
                Event::CData(text) => {
                    let decoded = self.reader.decoder().decode(text.as_ref())?.into_owned();
                    self.pending.push_back(XmlEvent::Characters(decoded));
                    break;
                }
                Event::Eof => {
                    self.pending.push_back(XmlEvent::EndDocument);
                    break;
                }
                Event::Decl(_) | Event::PI(_) | Event::Comment(_) | Event::DocType(_) => {
                    break;
                }
            }
        }

        if text
            .as_bytes()
            .iter()
            .all(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
        {
            Ok(XmlEvent::Whitespace(text))
        } else {
            Ok(XmlEvent::Characters(text))
        }
    }

    fn build_start_event(
        element: BytesStart<'_>,
        decoder: Decoder,
        resolver: &NamespaceResolver,
    ) -> Result<XmlEvent, quick_xml::Error> {
        let name = build_name_from_start(&element, resolver)?;
        let (attributes, namespace) = build_attributes(&element, decoder, resolver)?;
        Ok(XmlEvent::StartElement {
            name,
            attributes,
            namespace,
        })
    }
}

impl<R: Read> Iterator for EventReader<R> {
    type Item = Result<XmlEvent, quick_xml::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        match self.read_next() {
            Ok(XmlEvent::EndDocument) => {
                self.finished = true;
                Some(Ok(XmlEvent::EndDocument))
            }
            Ok(event) => Some(Ok(event)),
            Err(e) => {
                self.finished = true;
                Some(Err(e))
            }
        }
    }
}

fn build_name_from_start(
    element: &BytesStart<'_>,
    resolver: &NamespaceResolver,
) -> Result<OwnedName, quick_xml::Error> {
    let name = element.name();
    Ok(resolve_name(
        name.as_ref(),
        resolver.resolve_element(name).0,
    ))
}

fn build_name_from_end(
    element: &BytesEnd<'_>,
    resolver: &NamespaceResolver,
) -> Result<OwnedName, quick_xml::Error> {
    let name = element.name();
    Ok(resolve_name(
        name.as_ref(),
        resolver.resolve_element(name).0,
    ))
}

fn decode_text(text: BytesText<'_>) -> Result<String, quick_xml::Error> {
    Ok(text.xml_content(XmlVersion::Implicit1_0)?.into_owned())
}

fn decode_reference(reference: BytesRef<'_>) -> Result<String, quick_xml::Error> {
    if let Some(ch) = reference.resolve_char_ref()? {
        return Ok(ch.to_string());
    }
    let reference = reference.xml_content(XmlVersion::Implicit1_0)?;
    match reference.as_ref() {
        "lt" => return Ok("<".to_owned()),
        "gt" => return Ok(">".to_owned()),
        "amp" => return Ok("&".to_owned()),
        "apos" => return Ok("'".to_owned()),
        "quot" => return Ok("\"".to_owned()),
        _ => {}
    }
    let escaped = format!("&{reference};");
    Ok(unescape(&escaped)?.into_owned())
}

fn owned_str(raw: &[u8]) -> String {
    // Names are almost always short ASCII; avoid the general UTF-8 decoders.
    if raw.is_ascii() {
        // SAFETY: ASCII bytes are valid UTF-8.
        return unsafe { String::from_utf8_unchecked(raw.to_vec()) };
    }
    String::from_utf8_lossy(raw).into_owned()
}

fn resolve_name(raw: &[u8], namespace: ResolveResult<'_>) -> OwnedName {
    let mut name = split_qname(raw);
    if let ResolveResult::Bound(namespace) = namespace {
        name.namespace = Some(owned_str(namespace.as_ref()));
    }
    name
}

fn split_qname(raw: &[u8]) -> OwnedName {
    if let Some(idx) = raw.iter().position(|byte| *byte == b':') {
        OwnedName {
            local_name: owned_str(&raw[idx + 1..]),
            namespace: None,
            prefix: Some(owned_str(&raw[..idx])),
        }
    } else {
        OwnedName {
            local_name: owned_str(raw),
            namespace: None,
            prefix: None,
        }
    }
}

fn build_attributes(
    element: &BytesStart<'_>,
    decoder: Decoder,
    resolver: &NamespaceResolver,
) -> Result<(Vec<OwnedAttribute>, Namespace), quick_xml::Error> {
    let mut raw_attributes = element.attributes();
    let iter = raw_attributes.with_checks(false);
    let mut attributes = Vec::with_capacity(iter.size_hint().0);
    let mut namespace = Namespace::empty();
    for attr_result in iter {
        let attr = attr_result.map_err(quick_xml::Error::from)?;
        let value = attr
            .decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?
            .into_owned();
        // Keep declarations for generic XML round-trips, separate from properties.
        if attr.key.as_ref() == b"xmlns" {
            namespace.mappings.push((String::new(), value));
            continue;
        }
        if let Some(prefix) = attr.key.as_ref().strip_prefix(b"xmlns:") {
            namespace
                .mappings
                .push((String::from_utf8_lossy(prefix).into_owned(), value));
            continue;
        }
        let name = resolve_name(attr.key.as_ref(), resolver.resolve_attribute(attr.key).0);
        attributes.push(OwnedAttribute { name, value });
    }
    Ok((attributes, namespace))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_scoped_element_and_attribute_namespaces() {
        let xml = r#"<r xmlns="urn:word" xmlns:w="urn:word"><t w:val="Approved" val="plain">Report<t xmlns="urn:foreign" w:val="nested"/></t><t/></r>"#;
        let mut reader = EventReader::new(xml.as_bytes());
        let events: Vec<_> = reader.by_ref().map(Result::unwrap).collect();
        let starts: Vec<_> = events
            .iter()
            .filter_map(|event| match event {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => Some((name, attributes)),
                _ => None,
            })
            .collect();
        assert_eq!(starts.len(), 4);
        assert!(starts[0].1.is_empty());
        assert_eq!(starts[1].0.namespace.as_deref(), Some("urn:word"));
        let qualified = starts[1]
            .1
            .iter()
            .find(|a| a.name.prefix.as_deref() == Some("w"))
            .unwrap();
        assert_eq!(qualified.name.namespace.as_deref(), Some("urn:word"));
        let unqualified = starts[1]
            .1
            .iter()
            .find(|a| a.name.local_name == "val" && a.name.prefix.is_none())
            .unwrap();
        assert_eq!(unqualified.name.namespace, None);
        assert_eq!(starts[2].0.namespace.as_deref(), Some("urn:foreign"));
        assert_eq!(starts[3].0.namespace.as_deref(), Some("urn:word"));
        let ends: Vec<_> = events
            .iter()
            .filter_map(|event| match event {
                XmlEvent::EndElement { name } => Some(name),
                _ => None,
            })
            .collect();
        assert_eq!(
            ends.iter()
                .map(|name| name.namespace.as_deref())
                .collect::<Vec<_>>(),
            [
                Some("urn:foreign"),
                Some("urn:word"),
                Some("urn:word"),
                Some("urn:word")
            ]
        );
    }

    #[test]
    fn decodes_xml_entities_in_text_and_attributes() {
        let xml = r#"<w:t w:val="A &amp; B">A &amp; B</w:t>"#;
        let mut reader = EventReader::new(xml.as_bytes());

        let start = reader.next_event().unwrap();
        match start {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                assert_eq!(name.prefix.as_deref(), Some("w"));
                assert_eq!(name.local_name, "t");
                assert_eq!(attributes[0].name.prefix.as_deref(), Some("w"));
                assert_eq!(attributes[0].name.local_name, "val");
                assert_eq!(attributes[0].value, "A & B");
            }
            event => panic!("expected start element, got {event:?}"),
        }

        assert_eq!(
            reader.next_event().unwrap(),
            XmlEvent::Characters("A & B".to_string())
        );
        assert!(matches!(
            reader.next_event().unwrap(),
            XmlEvent::EndElement { name } if name.local_name == "t"
        ));
        assert_eq!(reader.next_event().unwrap(), XmlEvent::EndDocument);
    }
}
