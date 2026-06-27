use std::fmt;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;

/// OpenXML theme color identifier (`ST_ThemeColor`, ECMA-376 Part 1 §17.18.97).
///
/// A theme color tells Word to resolve the color from the document theme
/// (e.g. `accent1`) instead of using a fixed RGB value. The accompanying
/// `w:val` hex remains as a fallback for renderers that do not support themes.
///
/// ```
/// use docx_rs::ThemeColor;
///
/// assert_eq!(ThemeColor::Accent1.to_string(), "accent1");
/// assert_eq!(
///     "followedHyperlink".parse::<ThemeColor>().unwrap(),
///     ThemeColor::FollowedHyperlink,
/// );
/// ```
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThemeColor {
    /// `dark1`
    Dark1,
    /// `light1`
    Light1,
    /// `dark2`
    Dark2,
    /// `light2`
    Light2,
    /// `accent1`
    Accent1,
    /// `accent2`
    Accent2,
    /// `accent3`
    Accent3,
    /// `accent4`
    Accent4,
    /// `accent5`
    Accent5,
    /// `accent6`
    Accent6,
    /// `hyperlink`
    Hyperlink,
    /// `followedHyperlink`
    FollowedHyperlink,
    /// `none`
    None,
    /// `background1`
    Background1,
    /// `text1`
    Text1,
    /// `background2`
    Background2,
    /// `text2`
    Text2,
    /// Any token not recognized by this crate (preserves forward compatibility on read).
    Unsupported,
}

impl fmt::Display for ThemeColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ThemeColor::Dark1 => write!(f, "dark1"),
            ThemeColor::Light1 => write!(f, "light1"),
            ThemeColor::Dark2 => write!(f, "dark2"),
            ThemeColor::Light2 => write!(f, "light2"),
            ThemeColor::Accent1 => write!(f, "accent1"),
            ThemeColor::Accent2 => write!(f, "accent2"),
            ThemeColor::Accent3 => write!(f, "accent3"),
            ThemeColor::Accent4 => write!(f, "accent4"),
            ThemeColor::Accent5 => write!(f, "accent5"),
            ThemeColor::Accent6 => write!(f, "accent6"),
            ThemeColor::Hyperlink => write!(f, "hyperlink"),
            ThemeColor::FollowedHyperlink => write!(f, "followedHyperlink"),
            ThemeColor::None => write!(f, "none"),
            ThemeColor::Background1 => write!(f, "background1"),
            ThemeColor::Text1 => write!(f, "text1"),
            ThemeColor::Background2 => write!(f, "background2"),
            ThemeColor::Text2 => write!(f, "text2"),
            ThemeColor::Unsupported => write!(f, "unsupported"),
        }
    }
}

impl FromStr for ThemeColor {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dark1" => Ok(ThemeColor::Dark1),
            "light1" => Ok(ThemeColor::Light1),
            "dark2" => Ok(ThemeColor::Dark2),
            "light2" => Ok(ThemeColor::Light2),
            "accent1" => Ok(ThemeColor::Accent1),
            "accent2" => Ok(ThemeColor::Accent2),
            "accent3" => Ok(ThemeColor::Accent3),
            "accent4" => Ok(ThemeColor::Accent4),
            "accent5" => Ok(ThemeColor::Accent5),
            "accent6" => Ok(ThemeColor::Accent6),
            "hyperlink" => Ok(ThemeColor::Hyperlink),
            "followedHyperlink" => Ok(ThemeColor::FollowedHyperlink),
            "none" => Ok(ThemeColor::None),
            "background1" => Ok(ThemeColor::Background1),
            "text1" => Ok(ThemeColor::Text1),
            "background2" => Ok(ThemeColor::Background2),
            "text2" => Ok(ThemeColor::Text2),
            // Unknown tokens degrade gracefully rather than failing the whole parse,
            // matching the convention used by other ST_* enums in this crate.
            _ => Ok(ThemeColor::Unsupported),
        }
    }
}
