use super::{XMLBuilder, XmlEvent};
use std::io::Write;
use xml::writer::Result;

impl<W: Write> XMLBuilder<W> {
    /// Build XML declaration
    /// i.e. `<?xml version="1.0" encoding="UTF-8"?>`
    pub(crate) fn declaration(self, standalone: Option<bool>) -> Result<Self> {
        self.write(XmlEvent::StartDocument {
            version: super::XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_declaration() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.declaration(None)?.into_inner()?.into_inner();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?>"#
        );
        Ok(())
    }
}
