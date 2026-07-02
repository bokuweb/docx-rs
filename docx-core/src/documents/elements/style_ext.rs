//! Fork-only extension builders for [`Style`].
//!
//! Kept in a dedicated file that upstream `bokuweb/docx-rs` never edits, so
//! rebasing onto new releases produces zero conflicts here. Every method is a
//! pure delegation over the crate's own public API. See `FORK_CHANGES.md`.

use super::*;

impl Style {
    /// Applies shading to the style's run properties (`<w:rPr><w:shd/>`).
    ///
    /// Convenience delegator so callers never mutate the public
    /// `run_property` field directly.
    ///
    /// ```
    /// use docx_rs::*;
    /// let s = Style::new("MDTag", StyleType::Character)
    ///     .shading(Shading::new().shd_type(ShdType::Clear).fill("EEEEEE").color("auto"));
    /// assert!(String::from_utf8(s.build()).unwrap().contains(r#"w:fill="EEEEEE""#));
    /// ```
    pub fn shading(mut self, shading: Shading) -> Self {
        self.run_property = self.run_property.shading(shading);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // `super::*` only carries `elements`-level re-exports (e.g. `Shading`), not
    // `ShdType`/`StyleType` (crate::types) or the `BuildXML` trait (needed for
    // `.build()`), since neither is re-exported through `style.rs`/`shading.rs`.
    // Imported explicitly here rather than widening the module-level `use
    // super::*;` above, which must stay minimal per the fork's isolation
    // convention (see module doc comment).
    use crate::{documents::BuildXML, ShdType, StyleType};

    #[test]
    fn style_shading_delegates_to_run_property() {
        let s = Style::new("MDTag", StyleType::Character)
            .name("MD Tag")
            .shading(
                Shading::new()
                    .shd_type(ShdType::Clear)
                    .fill("EEEEEE")
                    .color("auto"),
            );
        let xml = String::from_utf8(s.build()).unwrap();
        assert!(xml.contains(r#"w:fill="EEEEEE""#), "got: {xml}");
        assert!(xml.contains(r#"w:val="clear""#), "got: {xml}");
    }
}
