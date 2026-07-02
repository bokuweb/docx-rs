//! Fork-only extension builders for [`Paragraph`].
//!
//! Isolated so upstream rebases never conflict here. Pure delegation over the
//! crate's public API. See `FORK_CHANGES.md`.

use super::*;

impl Paragraph {
    /// Sets the paragraph borders (`<w:pPr><w:pBdr/>`).
    ///
    /// Convenience delegator so callers never mutate the public `property`
    /// field directly.
    pub fn set_borders(mut self, borders: ParagraphBorders) -> Self {
        self.property = self.property.set_borders(borders);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::documents::build_xml::BuildXML;
    use crate::{BorderType, ParagraphBorderPosition};

    #[test]
    fn paragraph_set_borders_emits_pbdr() {
        let border = ParagraphBorder::new(ParagraphBorderPosition::Bottom)
            .val(BorderType::Single)
            .size(6)
            .space(1)
            .color("CCCCCC");
        let borders = ParagraphBorders::with_empty().set(border);
        let p = Paragraph::new().set_borders(borders);
        let xml = String::from_utf8(p.build()).unwrap();
        assert!(xml.contains("<w:pBdr>"), "got: {xml}");
        assert!(xml.contains(r#"w:color="CCCCCC""#), "got: {xml}");
    }
}
