use crate::documents::BuildXML;
use crate::xml_builder::*;

use crate::line_spacing_type::LineSpacingType;
use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LineSpacing {
    #[serde(skip_serializing_if = "Option::is_none")]
    line_rule: Option<LineSpacingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_lines: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_lines: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,
}

impl LineSpacing {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn line_rule(mut self, t: LineSpacingType) -> Self {
        self.line_rule = Some(t);
        self
    }

    pub fn before(mut self, before: u32) -> Self {
        self.before = Some(before);
        self
    }

    pub fn after(mut self, after: u32) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before_lines(mut self, before: u32) -> Self {
        self.before_lines = Some(before);
        self
    }

    pub fn after_lines(mut self, after: u32) -> Self {
        self.after_lines = Some(after);
        self
    }

    pub fn line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }
}

impl BuildXML for LineSpacing {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.line_spacing(
            self.before,
            self.after,
            self.line,
            self.before_lines,
            self.after_lines,
            self.line_rule,
        )
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
    fn test_spacing() {
        let b = LineSpacing::new()
            .line_rule(LineSpacingType::Auto)
            .line(100)
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:spacing w:line="100" w:lineRule="auto" />"#
        );
    }

    #[test]
    fn test_spacing_after_lines() {
        let b = LineSpacing::new()
            .line_rule(LineSpacingType::Auto)
            .after_lines(100)
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:spacing w:afterLines="100" w:lineRule="auto" />"#
        );
    }

    #[test]
    fn test_spacing_json() {
        let s = LineSpacing {
            line_rule: Some(LineSpacingType::Auto),
            before: None,
            after: None,
            before_lines: None,
            after_lines: None,
            line: Some(100),
        };
        assert_eq!(
            serde_json::to_string(&s).unwrap(),
            r#"{"lineRule":"Auto","line":100}"#
        );
    }
}
