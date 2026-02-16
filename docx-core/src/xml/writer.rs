use std::borrow::Cow;
use std::fmt;
use std::io::Write;
use std::marker::PhantomData;

use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::Writer as QuickWriter;

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
            Error::Io(err) => write!(f, "io error: {}", err),
            Error::Xml(err) => write!(f, "xml error: {}", err),
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
        }
    }
}

#[derive(Debug)]
struct ElementState {
    name: String,
    attributes: Vec<Attribute>,
    pending: bool,
}

pub struct EventWriter<W: Write> {
    writer: QuickWriter<W>,
    perform_escaping: bool,
    element_stack: Vec<ElementState>,
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
                let StartElement {
                    name, attributes, ..
                } = element;
                self.element_stack.push(ElementState {
                    name,
                    attributes,
                    pending: true,
                });
            }
            XmlEvent::EndElement => {
                let state = self.element_stack.pop().ok_or(Error::UnbalancedEndTag)?;
                if state.pending {
                    self.write_empty(state)?;
                } else {
                    self.write_closing(&state.name)?;
                }
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
        if let Some((name, attrs)) = self
            .element_stack
            .last_mut()
            .filter(|state| state.pending)
            .map(|state| {
                state.pending = false;
                (state.name.clone(), std::mem::take(&mut state.attributes))
            })
        {
            self.write_tag(name.as_str(), &attrs, b">")?;
        }
        Ok(())
    }

    fn write_empty(&mut self, state: ElementState) -> Result<()> {
        self.write_tag(state.name.as_str(), &state.attributes, b" />")
    }

    fn write_closing(&mut self, name: &str) -> Result<()> {
        let writer = self.writer.get_mut();
        writer.write_all(b"</").map_err(Error::from)?;
        writer.write_all(name.as_bytes()).map_err(Error::from)?;
        writer.write_all(b">").map_err(Error::from)
    }

    fn write_tag(&mut self, name: &str, attributes: &[Attribute], suffix: &[u8]) -> Result<()> {
        let writer = self.writer.get_mut();
        writer.write_all(b"<").map_err(Error::from)?;
        writer.write_all(name.as_bytes()).map_err(Error::from)?;
        for attr in attributes {
            writer.write_all(b" ").map_err(Error::from)?;
            writer
                .write_all(attr.name.as_bytes())
                .map_err(Error::from)?;
            writer.write_all(b"=\"").map_err(Error::from)?;
            writer
                .write_all(attr.value.as_bytes())
                .map_err(Error::from)?;
            writer.write_all(b"\"").map_err(Error::from)?;
        }
        writer.write_all(suffix).map_err(Error::from)
    }
}

#[derive(Clone, Debug)]
pub struct StartElement<'a> {
    name: String,
    attributes: Vec<Attribute>,
    _marker: PhantomData<&'a ()>,
}

#[derive(Clone, Debug)]
struct Attribute {
    name: String,
    value: String,
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
        StartElement {
            name: name.to_string(),
            attributes: Vec::new(),
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
    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.attributes.push(Attribute {
            name: name.to_string(),
            value: value.to_string(),
        });
        self
    }
}
