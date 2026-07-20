use std::borrow::Cow;
use std::fmt;
use std::io::Write;
use std::marker::PhantomData;

use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::Writer as QuickWriter;
use smallvec::SmallVec;

use super::common::XmlVersion;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Xml(quick_xml::Error),
    UnbalancedEndTag,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "io error: {err}"),
            Error::Xml(err) => write!(f, "xml error: {err}"),
            Error::UnbalancedEndTag => write!(f, "attempted to close more elements than opened"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Xml(err) => Some(err),
            Error::UnbalancedEndTag => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(value: quick_xml::Error) -> Self {
        Error::Xml(value)
    }
}

#[derive(Clone, Debug)]
pub struct EmitterConfig {
    pub write_document_declaration: bool,
    pub perform_escaping: bool,
    pub perform_indent: bool,
    pub line_separator: Cow<'static, str>,
}

impl Default for EmitterConfig {
    fn default() -> Self {
        Self {
            write_document_declaration: true,
            perform_escaping: true,
            perform_indent: false,
            line_separator: Cow::Borrowed("\n"),
        }
    }
}

impl EmitterConfig {
    pub fn create_writer<W: Write>(&self, writer: W) -> EventWriter<W> {
        EventWriter {
            writer: QuickWriter::new(writer),
            perform_escaping: self.perform_escaping,
            element_stack: Vec::new(),
            element_names: Vec::new(),
            pending_start: SmallVec::new(),
        }
    }
}

#[derive(Debug)]
struct ElementState {
    name_start: usize,
    name_len: usize,
    pending: bool,
}

pub struct EventWriter<W: Write> {
    writer: QuickWriter<W>,
    perform_escaping: bool,
    element_stack: Vec<ElementState>,
    element_names: Vec<u8>,
    pending_start: SmallVec<[u8; 128]>,
}

impl<W: Write> EventWriter<W> {
    pub fn write<'a, E>(&mut self, event: E) -> Result<()>
    where
        E: Into<XmlEvent<'a>>,
    {
        match event.into() {
            XmlEvent::StartDocument {
                version,
                encoding,
                standalone,
            } => {
                let standalone_text = standalone.map(|flag| if flag { "yes" } else { "no" });
                let decl = BytesDecl::new(version.as_str(), encoding, standalone_text);
                self.writer.write_event(Event::Decl(decl))?;
            }
            XmlEvent::StartElement(element) => {
                self.flush_pending()?;
                let name = &element.encoded[1..1 + element.name_len];
                let name_start = self.element_names.len();
                self.element_names.extend_from_slice(name);
                self.element_stack.push(ElementState {
                    name_start,
                    name_len: element.name_len,
                    pending: true,
                });
                self.pending_start = element.encoded;
            }
            XmlEvent::EndElement => {
                let state = self.element_stack.pop().ok_or(Error::UnbalancedEndTag)?;
                if state.pending {
                    let writer = self.writer.get_mut();
                    writer.write_all(&self.pending_start).map_err(Error::from)?;
                    writer.write_all(b" />").map_err(Error::from)?;
                    self.pending_start.clear();
                } else {
                    let name_end = state.name_start + state.name_len;
                    let writer = self.writer.get_mut();
                    writer.write_all(b"</").map_err(Error::from)?;
                    writer
                        .write_all(&self.element_names[state.name_start..name_end])
                        .map_err(Error::from)?;
                    writer.write_all(b">").map_err(Error::from)?;
                }
                self.element_names.truncate(state.name_start);
            }
            XmlEvent::Characters(text) => {
                self.flush_pending()?;
                let text_event = if self.perform_escaping {
                    BytesText::new(text.as_ref())
                } else {
                    BytesText::from_escaped(text.as_ref())
                };
                self.writer.write_event(Event::Text(text_event))?;
            }
        }
        Ok(())
    }

    pub fn into_inner(self) -> Result<W> {
        Ok(self.writer.into_inner())
    }

    pub fn inner_mut(&mut self) -> Result<&mut W> {
        Ok(self.writer.get_mut())
    }

    fn flush_pending(&mut self) -> Result<()> {
        if let Some(state) = self.element_stack.last_mut() {
            if state.pending {
                state.pending = false;
                let writer = self.writer.get_mut();
                writer.write_all(&self.pending_start).map_err(Error::from)?;
                writer.write_all(b">").map_err(Error::from)?;
                self.pending_start.clear();
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct StartElement<'a> {
    encoded: SmallVec<[u8; 128]>,
    name_len: usize,
    _marker: PhantomData<&'a ()>,
}

#[derive(Clone, Debug)]
pub enum XmlEvent<'a> {
    StartDocument {
        version: XmlVersion,
        encoding: Option<&'a str>,
        standalone: Option<bool>,
    },
    StartElement(StartElement<'a>),
    EndElement,
    Characters(Cow<'a, str>),
}

impl<'a> XmlEvent<'a> {
    pub fn start_element(name: &'a str) -> StartElement<'a> {
        let mut encoded = SmallVec::new();
        encoded.push(b'<');
        encoded.extend_from_slice(name.as_bytes());
        StartElement {
            encoded,
            name_len: name.len(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn end_element() -> XmlEvent<'static> {
        XmlEvent::EndElement
    }
}

impl<'a> From<StartElement<'a>> for XmlEvent<'a> {
    fn from(value: StartElement<'a>) -> Self {
        XmlEvent::StartElement(value)
    }
}

impl<'a> From<&'a str> for XmlEvent<'a> {
    fn from(value: &'a str) -> Self {
        XmlEvent::Characters(Cow::Borrowed(value))
    }
}

impl From<String> for XmlEvent<'static> {
    fn from(value: String) -> Self {
        XmlEvent::Characters(Cow::Owned(value))
    }
}

impl<'a> StartElement<'a> {
    /// Appends an attribute with an already escaped string value.
    ///
    /// The value is copied directly into the start tag and is not XML-escaped.
    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.encoded.push(b' ');
        self.encoded.extend_from_slice(name.as_bytes());
        self.encoded.extend_from_slice(b"=\"");
        self.encoded.extend_from_slice(value.as_bytes());
        self.encoded.push(b'"');
        self
    }

    /// Appends an attribute by formatting its value directly into the start tag.
    ///
    /// This avoids allocating an intermediate `String`. The formatted value is
    /// not XML-escaped.
    pub fn attr_display(mut self, name: &str, value: impl fmt::Display) -> Self {
        self.encoded.push(b' ');
        self.encoded.extend_from_slice(name.as_bytes());
        self.encoded.extend_from_slice(b"=\"");
        fmt::write(
            &mut AttributeValueWriter(&mut self.encoded),
            format_args!("{value}"),
        )
        .expect("writing to an in-memory attribute buffer cannot fail");
        self.encoded.push(b'"');
        self
    }
}

struct AttributeValueWriter<'a>(&'a mut SmallVec<[u8; 128]>);

impl fmt::Write for AttributeValueWriter<'_> {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        self.0.extend_from_slice(value.as_bytes());
        Ok(())
    }
}
