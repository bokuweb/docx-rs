use super::XMLBuilder;

use std::io::Write;

impl<W: Write> XMLBuilder<W> {
    // Build XML declaration
    // i.e. <?xml version="1.0" encoding="UTF-8"?>
    pub(crate) fn declaration(mut self, standalone: Option<bool>) -> Self {
        self.writer
            .write(super::XmlEvent::StartDocument {
                version: super::XmlVersion::Version10,
                encoding: Some("UTF-8"),
                standalone,
            })
            .expect("should write to buf");
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str;

    #[test]
    fn test_declaration() {
        let b = XMLBuilder::new(Vec::new());
        let r = b.declaration(None).into_inner();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?>"#
        );
    }
}
