use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RunProperty {
    // Sequential child elements

    // Sequence 1
    // 1. w:ins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<Insert>,
    // 2. w:del
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del: Option<Delete>,

    // Sequence 2
    // 1. w:rStyle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RunStyle>,
    // 2. w:rFonts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fonts: Option<RunFonts>,
    // 3. w:b
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<Bold>,
    // 4. w:bCs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold_cs: Option<BoldCs>,
    // 5. w:i
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<Italic>,
    // 6. w:iCs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic_cs: Option<ItalicCs>,
    // 7. w:caps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caps: Option<Caps>,
    // 8. w:smallCaps
    // TODO: Add SmallCaps type
    // 9. w:strike
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike: Option<Strike>,
    // 10. w:dstrike
    // TODO: Add Dstrike type
    // 11. w:outline
    // TODO: Add Outline type
    // 12. w:shadow
    // TODO: Add Shadow type
    // 13. w:emboss
    // TODO: Add Emboss type
    // 14. w:imprint
    // TODO: Add Imprint type
    // 15. w:noProof
    // TODO: Add NoProof type
    // 16. w:snapToGrid
    // TODO: Add SnapToGrid type
    // 17. w:vanish
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanish: Option<Vanish>,
    // 18. w:webHidden
    // TODO: Add WebHidden type
    // 19. w:color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    // 20. w:spacing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_spacing: Option<CharacterSpacing>,
    // 21. w:w
    // TODO: Add W type
    // 22. w:kern
    // TODO: Add Kern type
    // 23. w:position
    // TODO: Add Position type
    // 24. w:sz
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<Sz>,
    // 25. w:szCs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz_cs: Option<SzCs>,
    // 26. w:highlight
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<Highlight>,
    // 27. w:u
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<Underline>,
    // 28. w:effect
    // TODO: Add Effect type
    // 29. w:bdr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_border: Option<TextBorder>,
    // 30. w:shd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shading: Option<Shading>,
    // 31. w:fitText
    // TODO: Add FitText type
    // 32. w:vertAlign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vert_align: Option<VertAlign>,
    // 33. w:rtl
    // TODO: Add Rtl type
    // 34. w:cs
    // TODO: Add Cs type
    // 35. w:em
    // TODO: Add Em type
    // 36. w:lang
    // TODO: Add Lang type
    // 37. w:eastAsianLayout
    // TODO: Add EastAsianLayout type
    // 38. w:specVanish
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_vanish: Option<SpecVanish>,
    // 39. w:oMath
    // TODO: Add OMath type

    //Non runpropety element
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positional_tab: Option<PositionalTab>,
}

impl RunProperty {
    pub fn new() -> RunProperty {
        Default::default()
    }

    pub fn style(mut self, style_id: &str) -> Self {
        self.style = Some(RunStyle::new(style_id));
        self
    }

    pub fn size(mut self, size: usize) -> RunProperty {
        self.sz = Some(Sz::new(size));
        self.sz_cs = Some(SzCs::new(size));
        self
    }

    pub fn spacing(mut self, spacing: i32) -> RunProperty {
        self.character_spacing = Some(CharacterSpacing::new(spacing));
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> RunProperty {
        self.color = Some(Color::new(color));
        self
    }

    pub fn highlight(mut self, color: impl Into<String>) -> RunProperty {
        self.highlight = Some(Highlight::new(color));
        self
    }

    pub fn vert_align(mut self, a: VertAlignType) -> Self {
        self.vert_align = Some(VertAlign::new(a));
        self
    }

    pub fn bold(mut self) -> RunProperty {
        self.bold = Some(Bold::new());
        self.bold_cs = Some(BoldCs::new());
        self
    }

    pub fn disable_bold(mut self) -> RunProperty {
        self.bold = Some(Bold::new().disable());
        self.bold_cs = Some(BoldCs::new().disable());
        self
    }

    pub fn caps(mut self) -> RunProperty {
        self.caps = Some(Caps::new());
        self
    }

    pub fn italic(mut self) -> RunProperty {
        self.italic = Some(Italic::new());
        self.italic_cs = Some(ItalicCs::new());
        self
    }

    pub fn strike(mut self) -> RunProperty {
        self.strike = Some(Strike::new());
        self
    }

    pub fn disable_italic(mut self) -> RunProperty {
        self.italic = Some(Italic::new().disable());
        self.italic_cs = Some(ItalicCs::new().disable());
        self
    }

    pub fn underline(mut self, line_type: impl Into<String>) -> RunProperty {
        self.underline = Some(Underline::new(line_type));
        self
    }

    pub fn vanish(mut self) -> RunProperty {
        self.vanish = Some(Vanish::new());
        self
    }

    pub fn spec_vanish(mut self) -> RunProperty {
        self.spec_vanish = Some(SpecVanish::new());
        self
    }

    pub fn fonts(mut self, font: RunFonts) -> RunProperty {
        self.fonts = Some(font);
        self
    }

    pub fn text_border(mut self, b: TextBorder) -> Self {
        self.text_border = Some(b);
        self
    }

    pub fn delete(mut self, d: Delete) -> Self {
        self.del = Some(d);
        self
    }

    pub fn insert(mut self, i: Insert) -> Self {
        self.ins = Some(i);
        self
    }

    pub fn ptab(mut self, ptab: PositionalTab) -> Self {
        self.positional_tab = Some(ptab);
        self
    }

    pub fn shading(mut self, s: Shading) -> Self {
        self.shading = Some(s);
        self
    }
}

impl BuildXML for RunProperty {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_run_property()
            .add_optional_child(&self.ins)
            .add_optional_child(&self.del)
            .add_optional_child(&self.style)
            .add_optional_child(&self.fonts)
            .add_optional_child(&self.bold)
            .add_optional_child(&self.bold_cs)
            .add_optional_child(&self.italic)
            .add_optional_child(&self.italic_cs)
            .add_optional_child(&self.caps)
            .add_optional_child(&self.strike)
            .add_optional_child(&self.vanish)
            .add_optional_child(&self.color)
            .add_optional_child(&self.character_spacing)
            .add_optional_child(&self.sz)
            .add_optional_child(&self.sz_cs)
            .add_optional_child(&self.highlight)
            .add_optional_child(&self.underline)
            .add_optional_child(&self.text_border)
            .add_optional_child(&self.shading)
            .add_optional_child(&self.vert_align)
            .add_optional_child(&self.spec_vanish)
            .add_optional_child(&self.positional_tab)
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_size() {
        let c = RunProperty::new().size(10).color("FFFFFF");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:color w:val="FFFFFF" /><w:sz w:val="10" /><w:szCs w:val="10" /></w:rPr>"#
        );
    }

    #[test]
    fn test_highlight() {
        let c = RunProperty::new().highlight("FFFFFF");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:highlight w:val="FFFFFF" /></w:rPr>"#
        );
    }

    #[test]
    fn test_bold() {
        let c = RunProperty::new().bold();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:b /><w:bCs /></w:rPr>"#
        );
    }

    #[test]
    fn test_strike() {
        let c = RunProperty::new().strike();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:strike /></w:rPr>"#
        );
    }

    #[test]
    fn test_underline() {
        let c = RunProperty::new().underline("single");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:u w:val="single" /></w:rPr>"#
        );
    }

    #[test]
    fn test_vanish() {
        let c = RunProperty::new().vanish();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:vanish /></w:rPr>"#
        );
    }

    #[test]
    fn test_run_fonts() {
        let c = RunProperty::new().fonts(RunFonts::new().east_asia("Hiragino"));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:rFonts w:eastAsia="Hiragino" /></w:rPr>"#
        );
    }

    #[test]
    fn test_character_spacing() {
        let c = RunProperty::new().spacing(20);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:spacing w:val="20" /></w:rPr>"#
        );
    }

    #[test]
    fn test_ptab() {
        let c = RunProperty::new().ptab(PositionalTab::new(
            PositionalTabAlignmentType::Left,
            PositionalTabRelativeTo::Margin,
            TabLeaderType::None,
        ));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:ptab w:alignment="left" w:relativeTo="margin" w:leader="none" /></w:rPr>"#
        );
    }

    #[test]
    fn test_character_shading() {
        let c = RunProperty::new().shading(
            Shading::new()
                .shd_type(ShdType::Clear)
                .fill("FFFFFF")
                .color("auto"),
        );
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:shd w:val="clear" w:color="auto" w:fill="FFFFFF" /></w:rPr>"#
        );
    }
}
