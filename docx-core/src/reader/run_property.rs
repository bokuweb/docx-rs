use std::io::Read;
use std::str::FromStr;

use crate::{ThemeColor, VertAlignType};

use super::*;

// Parse a `<w:color>` element, including the optional theme attributes.
//
// Attributes are read by local name rather than position: Word may emit them
// in any order, and a theme-only color can omit `w:val` (we fall back to
// "auto"). Unknown themeColor tokens degrade to `ThemeColor::Unsupported`
// via its `FromStr`, so a future ST_ThemeColor value never breaks reading.
fn read_color(rp: RunProperty, attributes: &[OwnedAttribute]) -> RunProperty {
    let val = read(attributes, "val").unwrap_or_else(|| "auto".to_string());
    let mut rp = rp.color(val);
    if let Some(tc) = read(attributes, "themeColor") {
        if let Ok(theme) = ThemeColor::from_str(&tc) {
            rp = rp.theme_color(theme);
        }
    }
    if let Some(ts) = read(attributes, "themeShade") {
        rp = rp.theme_shade(ts);
    }
    if let Some(tt) = read(attributes, "themeTint") {
        rp = rp.theme_tint(tt);
    }
    rp
}

fn read_run_fonts(attributes: &[OwnedAttribute]) -> Result<RunFonts, ReaderError> {
    let mut f = RunFonts::new();
    for a in attributes {
        let local_name = &a.name.local_name;
        match local_name.as_str() {
            "asciiTheme" => {
                f = f.ascii_theme(&a.value);
            }
            "eastAsiaTheme" => {
                f = f.east_asia_theme(&a.value);
            }
            "hAnsiTheme" => {
                f = f.hi_ansi_theme(&a.value);
            }
            "cstheme" => {
                f = f.cs_theme(&a.value);
            }
            "ascii" => {
                f = f.ascii(&a.value);
            }
            "eastAsia" => {
                f = f.east_asia(&a.value);
            }
            "hAnsi" => {
                f = f.hi_ansi(&a.value);
            }
            "cs" => {
                f = f.cs(&a.value);
            }
            "hint" => {
                f = f.hint(&a.value);
            }
            _ => {}
        }
    }
    Ok(f)
}

impl ElementReader for RunProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut rp = RunProperty::new();
        loop {
            let e = r.next_event();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    ignore::ignore_element(e.clone(), XMLElement::RunPropertyChange, r);

                    match e {
                        XMLElement::RunStyle => {
                            if let Some(v) = read_val(&attributes) {
                                rp = rp.style(&v);
                            }
                        }
                        XMLElement::Bold => {
                            if !read_bool(&attributes) {
                                rp = rp.disable_bold();
                                continue;
                            }
                            rp = rp.bold();
                        }
                        XMLElement::Caps => {
                            if !read_bool(&attributes) {
                                rp.caps = Some(Caps::new().disable());
                                continue;
                            }
                            rp = rp.caps();
                        }
                        XMLElement::PTab => {
                            if let Ok(v) = PositionalTab::read(r, &attributes) {
                                rp = rp.ptab(v)
                            }
                        }
                        XMLElement::Highlight => rp = rp.highlight(attributes[0].value.clone()),
                        XMLElement::Strike => {
                            if !read_bool(&attributes) {
                                rp.strike = Some(Strike::new().disable());
                                continue;
                            }
                            rp = rp.strike();
                        }
                        XMLElement::Dstrike => {
                            if !read_bool(&attributes) {
                                rp.dstrike = Some(Dstrike::new().disable());
                                continue;
                            }
                            rp = rp.dstrike();
                        }
                        XMLElement::VertAlign => {
                            if let Ok(v) = VertAlignType::from_str(&attributes[0].value) {
                                rp = rp.vert_align(v)
                            }
                        }
                        XMLElement::Color => rp = read_color(rp, &attributes),
                        XMLElement::Size => {
                            rp = rp.size(f64::from_str(&attributes[0].value)? as usize)
                        }
                        XMLElement::Spacing => {
                            if let Some(v) = read_val(&attributes) {
                                if let Ok(s) = f64::from_str(&v) {
                                    rp = rp.spacing(s as i32)
                                }
                            }
                        }
                        XMLElement::W => {
                            if let Some(v) = read_val(&attributes) {
                                if let Ok(scale) = i32::from_str(&v) {
                                    rp = rp.stretch(scale);
                                }
                            }
                        }
                        XMLElement::FitText => {
                            if let Some(v) = read_val(&attributes) {
                                if let Ok(val) = usize::from_str(&v) {
                                    let id = read(&attributes, "id")
                                        .and_then(|id| u32::from_str(&id).ok());
                                    rp = rp.fit_text(val, id);
                                }
                            }
                        }
                        XMLElement::RunFonts => {
                            if let Ok(f) = read_run_fonts(&attributes) {
                                rp = rp.fonts(f);
                            }
                        }
                        XMLElement::Underline => rp = rp.underline(attributes[0].value.clone()),
                        XMLElement::Italic => {
                            if !read_bool(&attributes) {
                                rp = rp.disable_italic();
                                continue;
                            }
                            rp = rp.italic();
                        }
                        XMLElement::Shading => {
                            if let Ok(shd) = Shading::read(r, &attributes) {
                                rp = rp.shading(shd);
                            }
                        }
                        XMLElement::Vanish => rp = rp.vanish(),
                        XMLElement::SpecVanish => rp = rp.spec_vanish(),
                        XMLElement::TextBorder => {
                            if let Ok(attr) = read_border(&attributes) {
                                let mut border = TextBorder::new()
                                    .border_type(attr.border_type)
                                    .color(attr.color);
                                if let Some(size) = attr.size {
                                    border = border.size(size as usize);
                                };
                                rp = rp.text_border(border);
                                continue;
                            }
                        }
                        XMLElement::Insert => {
                            if let Ok(ins) = Insert::read(r, &attributes) {
                                rp = rp.insert(ins);
                            }
                        }
                        XMLElement::Delete => {
                            if let Ok(del) = Delete::read(r, &attributes) {
                                rp = rp.delete(del);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::RunProperty {
                        return Ok(rp);
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

    fn read_properties(children: &str) -> RunProperty {
        let xml = format!(
            r#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">{children}</w:rPr>"#
        );
        RunProperty::read(&mut EventReader::new(xml.as_bytes()), &[]).unwrap()
    }

    #[test]
    fn read_character_stretch_preserves_compression_neutral_and_expansion() {
        for scale in [75, 100, 125, 160] {
            let properties = read_properties(&format!(
                r#"<w:color w:val="123456"/><w:w w:val="{scale}"/><w:b/>"#
            ));
            assert_eq!(properties.stretch, Some(Stretch::new(scale)));
            assert_eq!(properties.color, Some(Color::new("123456")));
            assert_eq!(properties.bold, Some(Bold::new()));
        }
    }

    #[test]
    fn read_character_stretch_finds_value_after_other_attributes() {
        let properties = read_properties(r#"<w:w w:unused="ignored" w:val="125"/>"#);
        assert_eq!(properties.stretch, Some(Stretch::new(125)));
    }

    #[test]
    fn read_character_stretch_ignores_absent_or_invalid_values() {
        for element in [
            "",
            "<w:w/>",
            r#"<w:w w:val="invalid"/>"#,
            r#"<w:w w:val="125.5"/>"#,
            r#"<w:w w:val="2147483648"/>"#,
        ] {
            let properties = read_properties(&format!("{element}<w:i/>"));
            assert_eq!(properties.stretch, None);
            assert_eq!(properties.italic, Some(Italic::new()));
        }
    }
}
