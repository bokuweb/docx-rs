use std::fmt;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::Serialize;

use super::errors;
use std::str::FromStr;

// @See: 20.4.3.4 ST_RelFromH (Horizontal Relative Positioning)
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub enum RelativeFromHType {
    /// Specifies that the horizontal positioning shall be
    /// relative to the position of the anchor within its run
    /// content.
    Character,
    /// Specifies that the horizontal positioning shall be
    /// relative to the extents of the column which contains its
    /// anchor.
    Column,
    /// Specifies that the horizontal positioning shall be
    /// relative to the inside margin of the current page (the
    /// left margin on odd pages, right on even pages).
    InsideMargin,
    /// Specifies that the horizontal positioning shall be
    /// relative to the left margin of the page.
    LeftMargin,
    /// Specifies that the horizontal positioning shall be
    /// relative to the page margins.
    Margin,
    /// Specifies that the horizontal positioning shall be
    /// relative to the outside margin of the current page (the
    /// right margin on odd pages, left on even pages).
    OutsizeMargin,
    /// Specifies that the horizontal positioning shall be
    /// relative to the edge of the page.
    Page,
    /// Specifies that the horizontal positioning shall be
    /// relative to the right margin of the page.
    RightMargin,
}

impl Default for RelativeFromHType {
    fn default() -> Self {
        Self::Margin
    }
}

impl fmt::Display for RelativeFromHType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RelativeFromHType::Character => write!(f, "character"),
            RelativeFromHType::Column => write!(f, "column"),
            RelativeFromHType::InsideMargin => write!(f, "insideMargin"),
            RelativeFromHType::LeftMargin => write!(f, "leftMargin"),
            RelativeFromHType::Margin => write!(f, "margin"),
            RelativeFromHType::OutsizeMargin => write!(f, "outsizeMargin"),
            RelativeFromHType::Page => write!(f, "page"),
            RelativeFromHType::RightMargin => write!(f, "rightMargin"),
        }
    }
}

impl FromStr for RelativeFromHType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "character" => Ok(RelativeFromHType::Character),
            "column" => Ok(RelativeFromHType::Column),
            "insideMargin" => Ok(RelativeFromHType::InsideMargin),
            "leftMargin" => Ok(RelativeFromHType::LeftMargin),
            "margin" => Ok(RelativeFromHType::Margin),
            "outsizeMargin" => Ok(RelativeFromHType::OutsizeMargin),
            "page" => Ok(RelativeFromHType::Page),
            "rightMargin" => Ok(RelativeFromHType::RightMargin),
            _ => Ok(RelativeFromHType::Margin),
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub enum RelativeFromVType {
    BottomMargin,
    InsideMargin,
    Line,
    Margin,
    OutsizeMargin,
    Page,
    Paragraph,
    TopMargin,
}

impl Default for RelativeFromVType {
    fn default() -> Self {
        Self::Margin
    }
}

impl fmt::Display for RelativeFromVType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RelativeFromVType::BottomMargin => write!(f, "bottomMargin"),
            RelativeFromVType::InsideMargin => write!(f, "insideMargin"),
            RelativeFromVType::Line => write!(f, "line"),
            RelativeFromVType::Margin => write!(f, "margin"),
            RelativeFromVType::OutsizeMargin => write!(f, "outsideMargin"),
            RelativeFromVType::Page => write!(f, "page"),
            RelativeFromVType::Paragraph => write!(f, "paragraph"),
            RelativeFromVType::TopMargin => write!(f, "topMargin"),
        }
    }
}

impl FromStr for RelativeFromVType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bottomMargin" => Ok(RelativeFromVType::BottomMargin),
            "insideMargin" => Ok(RelativeFromVType::InsideMargin),
            "line" => Ok(RelativeFromVType::Line),
            "margin" => Ok(RelativeFromVType::Margin),
            "outsizeMargin" => Ok(RelativeFromVType::OutsizeMargin),
            "page" => Ok(RelativeFromVType::Page),
            "paragraph" => Ok(RelativeFromVType::Paragraph),
            "topMargin" => Ok(RelativeFromVType::TopMargin),
            _ => Ok(RelativeFromVType::Margin),
        }
    }
}
