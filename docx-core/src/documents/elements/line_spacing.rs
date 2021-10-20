use crate::documents::BuildXML;
use crate::xml_builder::*;

use crate::line_spacing_type::LineSpacingType;
use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LineSpacing {
    #[serde(skip_serializing_if = "Option::is_none")]
    line_rule: Option<LineSpacingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,
}

impl LineSpacing {
    pub fn new(spacing: Option<LineSpacingType>) -> Self {
        Self {
            line_rule: spacing,
            before: None,
            after: None,
            line: None,
        }
    }
    pub fn before(mut self, before: Option<u32>) -> Self {
        self.before = before;
        self
    }
    pub fn after(mut self, after: Option<u32>) -> Self {
        self.after = after;
        self
    }
    pub fn line(mut self, line: Option<u32>) -> Self {
        self.line = line;
        self
    }
}

impl BuildXML for LineSpacing {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.line_spacing(self.before, self.after, self.line, self.line_rule)
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
        let b = LineSpacing::new(Some(LineSpacingType::Auto))
            .line(Some(100))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:spacing w:line="100" w:lineRule="auto" />"#
        );
    }

    #[test]
    fn test_spacing_json() {
        let s = LineSpacing {
            line_rule: Some(LineSpacingType::Auto),
            before: None,
            after: None,
            line: Some(100),
        };
        assert_eq!(
            serde_json::to_string(&s).unwrap(),
            r#"{"lineRule":"Auto","line":100}"#
        );
    }
}
