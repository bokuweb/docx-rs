use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::ThemeColor;
use crate::xml_builder::*;

/// A `<w:color>` color property.
///
/// `val` is an `RRGGBB` hex string (or `auto`). The optional theme fields let
/// the color reference a document-theme color instead of being locked to the
/// hex value: when present they emit `w:themeColor` / `w:themeShade` /
/// `w:themeTint`, and Word resolves the color from the active theme while
/// `val` stays as a fallback for renderers that ignore themes.
///
/// ```
/// use docx_rs::{Color, ThemeColor};
///
/// // Plain hex color (unchanged behavior).
/// let c = Color::new("2E74B5");
///
/// // Theme-aware color: "use accent1, shaded to BF", with 2E74B5 as fallback.
/// let c = Color::new("2E74B5")
///     .theme_color(ThemeColor::Accent1)
///     .theme_shade("BF");
/// ```
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Color {
    val: String,
    #[serde(default)]
    theme_color: Option<String>,
    #[serde(default)]
    theme_shade: Option<String>,
    #[serde(default)]
    theme_tint: Option<String>,
}

impl Color {
    /// Creates a color from an `RRGGBB` hex string (or `auto`).
    pub fn new(val: impl Into<String>) -> Color {
        Color {
            val: val.into(),
            theme_color: None,
            theme_shade: None,
            theme_tint: None,
        }
    }

    /// Sets the theme color reference (`w:themeColor`).
    pub fn theme_color(mut self, theme_color: ThemeColor) -> Color {
        // Stored as the serialized ST_ThemeColor token (e.g. "accent1") so the
        // builder and reader share a single string representation; the typed
        // `ThemeColor` enum is the ergonomic boundary for callers.
        self.theme_color = Some(theme_color.to_string());
        self
    }

    /// Sets the theme shade modifier (`w:themeShade`), a hex byte such as `"BF"`.
    pub fn theme_shade(mut self, theme_shade: impl Into<String>) -> Color {
        self.theme_shade = Some(theme_shade.into());
        self
    }

    /// Sets the theme tint modifier (`w:themeTint`), a hex byte such as `"99"`.
    pub fn theme_tint(mut self, theme_tint: impl Into<String>) -> Color {
        self.theme_tint = Some(theme_tint.into());
        self
    }
}

impl BuildXML for Color {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .color_with_theme(
                &self.val,
                self.theme_color.as_deref(),
                self.theme_shade.as_deref(),
                self.theme_tint.as_deref(),
            )?
            .into_inner()
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Preserve the existing JSON shape (a bare hex string). Theme metadata
        // is not represented in JSON to avoid a breaking format change; it is
        // fully round-tripped through the docx/XML path, which is what consumers
        // generating documents rely on.
        serializer.serialize_str(&self.val)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_build() {
        let c = Color::new("FFFFFF");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:color w:val="FFFFFF" />"#);
    }

    #[test]
    fn test_build_with_theme_color() {
        let c = Color::new("2E74B5")
            .theme_color(ThemeColor::Accent1)
            .theme_shade("BF");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:color w:val="2E74B5" w:themeColor="accent1" w:themeShade="BF" />"#
        );
    }

    #[test]
    fn test_build_with_theme_tint() {
        let c = Color::new("2E74B5")
            .theme_color(ThemeColor::Accent2)
            .theme_tint("99");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:color w:val="2E74B5" w:themeColor="accent2" w:themeTint="99" />"#
        );
    }
}
