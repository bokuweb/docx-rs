use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

// https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.frameproperties?view=openxml-3.0.1
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct FrameProperty {
    /// Frame Height
    /// Represents the following attribute in the schema: w:h
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u32>,
    /// Frame Height Type
    /// Represents the following attribute in the schema: w:hRule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_rule: Option<String>,
    /// Frame Horizontal Positioning Base
    /// Represents the following attribute in the schema: w:hAnchor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_anchor: Option<String>,
    /// Horizontal Frame Padding
    /// Represents the following attribute in the schema: w:hSpace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_space: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_space: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u32>,
    /// Text Wrapping Around Frame
    /// Represents the following attribute in the schema: w:wrap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<String>,
    /// Absolute Horizontal Position
    /// Represents the following attribute in the schema: w:x
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    /// Relative Horizontal Position
    /// Represents the following attribute in the schema: w:xAlign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_align: Option<String>,
    /// Absolute Vertical Position
    /// Represents the following attribute in the schema: w:y
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
    /// Relative Vertical Position
    /// Represents the following attribute in the schema: w:yAlign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_align: Option<String>,
}

impl FrameProperty {
    pub fn new() -> FrameProperty {
        Default::default()
    }

    pub fn wrap(mut self, wrap: impl Into<String>) -> Self {
        self.wrap = Some(wrap.into());
        self
    }

    pub fn v_anchor(mut self, anchor: impl Into<String>) -> Self {
        self.v_anchor = Some(anchor.into());
        self
    }

    pub fn h_anchor(mut self, anchor: impl Into<String>) -> Self {
        self.h_anchor = Some(anchor.into());
        self
    }

    pub fn x_align(mut self, align: impl Into<String>) -> Self {
        self.x_align = Some(align.into());
        self
    }

    pub fn y_align(mut self, align: impl Into<String>) -> Self {
        self.y_align = Some(align.into());
        self
    }

    pub fn x(mut self, x: i32) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.y = Some(y);
        self
    }
}

impl BuildXML for FrameProperty {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.frame_property(self).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_q_format() {
        let c = FrameProperty::new().wrap("none");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:framePr w:wrap="none" />"#);
    }
}
