#[macro_use]
mod macros;

mod comments;
mod comments_extended;
mod core_properties;
mod custom_properties;
mod declaration;
mod document;
mod drawing;
mod elements;
mod fonts;
mod footer;
mod footnotes;
mod header;
mod numbering;
mod pic;
mod properties;
mod relationship;
mod settings;

mod styles;

use crate::BuildXML;

use crate::xml::common::XmlVersion;
use crate::xml::writer::{EmitterConfig, EventWriter, Result};
use std::io::Write;
use std::str;

pub use crate::xml::writer::XmlEvent;

pub struct XMLBuilder<W: Write> {
    writer: EventWriter<W>,
}

impl<W: Write> From<EventWriter<W>> for XMLBuilder<W> {
    fn from(writer: EventWriter<W>) -> Self {
        Self { writer }
    }
}

impl<W: Write> XMLBuilder<W> {
    pub(crate) fn new(writer: W) -> Self {
        let config = EmitterConfig {
            write_document_declaration: false,
            perform_escaping: false,
            perform_indent: false,
            line_separator: "".into(),
            ..Default::default()
        };
        let writer = config.create_writer(writer);
        XMLBuilder { writer }
    }

    #[inline]
    pub(crate) fn write<'e>(mut self, event: impl Into<XmlEvent<'e>>) -> Result<Self> {
        self.writer.write(event)?;
        Ok(self)
    }

    #[inline(always)]
    pub(crate) fn apply<F: FnOnce(Self) -> Result<Self>>(self, f: F) -> Result<Self> {
        f(self)
    }

    /// If `clause` argument is `true` or `Some(true)`, continue building XML.
    ///
    /// ```ignore
    /// XMLBuilder::from(stream)
    ///   .apply_if(self.keep_lines, |b| b.keep_lines())?
    /// ```
    #[inline(always)]
    pub(crate) fn apply_if<F: FnOnce(Self) -> Result<Self>>(
        self,
        clause: impl Into<Option<bool>>,
        f: F,
    ) -> Result<Self> {
        if clause.into() == Some(true) {
            f(self)
        } else {
            Ok(self)
        }
    }

    /// If `opt` argument is `Some`, continue building XML.
    ///
    /// ```ignore
    /// XMLBuilder::from(stream)
    ///   .apply_opt(self.config.language.as_ref(), |v, b| b.dc_language(v))?
    /// ```
    #[inline(always)]
    pub(crate) fn apply_opt<T, F: FnOnce(T, Self) -> Result<Self>>(
        self,
        opt: Option<T>,
        f: F,
    ) -> Result<Self> {
        if let Some(it) = opt {
            f(it, self)
        } else {
            Ok(self)
        }
    }

    #[inline(always)]
    pub(crate) fn apply_each<T, I: IntoIterator<Item = T>, F: Fn(T, Self) -> Result<Self>>(
        mut self,
        items: I,
        f: F,
    ) -> Result<Self> {
        for item in items.into_iter() {
            self = f(item, self)?;
        }
        Ok(self)
    }

    /// Pass the XML writer to the child, then get it back.
    ///
    /// Affects indentation as well as the stack of currently opened tags.
    pub(crate) fn add_child(self, child: &impl BuildXML) -> Result<Self> {
        Ok(child.build_to(self.writer)?.into())
    }

    pub(crate) fn add_optional_child(self, child: &Option<impl BuildXML>) -> Result<Self> {
        match child {
            Some(ch) => self.add_child(ch),
            None => Ok(self),
        }
    }

    pub(crate) fn add_children(mut self, children: &[impl BuildXML]) -> Result<Self> {
        for ch in children {
            self = self.add_child(ch)?;
        }
        Ok(self)
    }

    /// Close the previously opened tag
    pub(crate) fn close(self) -> Result<Self> {
        self.write(XmlEvent::end_element())
    }

    /// Write a plain text.
    ///
    /// Finishes opened tags if there are any.
    pub(crate) fn plain_text(self, t: &str) -> Result<Self> {
        self.write(t)
    }

    /// Prepare the internal raw stream for direct access.
    ///
    /// Finishes opened tags if there are any.
    ///
    /// ```ignore
    /// let mut b = XMLBuilder::from(stream);
    ///  write!(b.inner_mut()?, "{}", self.0)?;
    ///  b.into_inner()
    /// ```
    pub(crate) fn inner_mut(&mut self) -> Result<&mut W> {
        self.writer.write("")?; // closes non-finished tags
        Ok(self.writer.inner_mut()?)
    }

    /// Unwraps this `XmlBuilder`, returning the underlying writer.
    ///
    /// The return type is intentionally `Result`, to simplify chaining.
    pub(crate) fn into_inner(self) -> Result<EventWriter<W>> {
        Ok(self.writer)
    }
}

// specific helpers, which probably should be somewhere else
impl<W: Write> XMLBuilder<W> {
    /// Build types element
    ///
    /// i.e. `<Types xmlns="http://...">`
    pub(crate) fn open_types(self, uri: &str) -> Result<Self> {
        self.write(XmlEvent::start_element("Types").attr("xmlns", uri))
    }

    /// Build Override element
    ///
    /// i.e. `<Override PartName="/_rels/.rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>`
    pub(crate) fn add_override(self, name: &str, content_type: &str) -> Result<Self> {
        self.write(
            XmlEvent::start_element("Override")
                .attr("PartName", name)
                .attr("ContentType", content_type),
        )?
        .close()
    }

    pub(crate) fn add_default(self, name: &str, extension: &str) -> Result<Self> {
        self.write(
            XmlEvent::start_element("Default")
                .attr("ContentType", extension)
                .attr("Extension", name),
        )?
        .close()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_types() -> Result<()> {
        let r = XMLBuilder::new(Vec::new())
            .open_types("http://example")?
            .plain_text("child")?
            .close()?
            .into_inner()?
            .into_inner()?;
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<Types xmlns="http://example">child</Types>"#
        );
        Ok(())
    }
}
