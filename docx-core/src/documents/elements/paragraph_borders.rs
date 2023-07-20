use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphBorder {
    position: ParagraphBorderPosition,
    pub val: BorderType,
    pub size: usize,
    pub space: usize,
    pub color: String,
    // pub shadow: Option<bool>,
    // pub theme_color: Option<String>,
    // pub theme_shade: Option<String>,
    // pub theme_tint: Option<String>,
    // pub frame: Option<bool>,
}

impl ParagraphBorder {
    pub fn new(position: ParagraphBorderPosition) -> Self {
        ParagraphBorder {
            position,
            val: BorderType::Single,
            size: 2,
            space: 0,
            color: "auto".to_owned(),
            // shadow: None,
            // theme_color: None,
            // theme_shade: None,
            // theme_tint: None,
            // frame: None,
        }
    }
    pub fn val(mut self, val: BorderType) -> Self {
        self.val = val;
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn space(mut self, space: usize) -> Self {
        self.space = space;
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    // pub fn shadow(mut self, shadow: bool) -> Self {
    //     self.shadow = Some(shadow);
    //     self
    // }
    //
    // pub fn theme_color(mut self, theme_color: impl Into<String>) -> Self {
    //     self.theme_color = Some(theme_color.into());
    //     self
    // }
    //
    // pub fn theme_shade(mut self, theme_shade: impl Into<String>) -> Self {
    //     self.theme_shade = Some(theme_shade.into());
    //     self
    // }
    //
    // pub fn theme_tint(mut self, theme_tint: impl Into<String>) -> Self {
    //     self.theme_tint = Some(theme_tint.into());
    //     self
    // }
    //
    // pub fn frame(mut self, frame: bool) -> Self {
    //     self.frame = Some(frame);
    //     self
    // }
}

impl BuildXML for ParagraphBorder {
    fn build(&self) -> Vec<u8> {
        let base = XMLBuilder::new();
        let base = {
            let val = self.val.to_string();
            let space = self.space.to_string();
            let size = self.size.to_string();
            match self.position {
                ParagraphBorderPosition::Top => base.paragraph_border_top(&val, &space, &size, &self.color),
                ParagraphBorderPosition::Left => base.paragraph_border_left(&val, &space, &size, &self.color),
                ParagraphBorderPosition::Bottom => base.paragraph_border_bottom(&val, &space, &size, &self.color),
                ParagraphBorderPosition::Right => base.paragraph_border_right(&val, &space, &size, &self.color),
                ParagraphBorderPosition::Between => base.paragraph_border_between(&val, &space, &size, &self.color),
                ParagraphBorderPosition::Bar => base.paragraph_border_bar(&val, &space, &size, &self.color),
            }
        };
        base.build()
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphBorders {
    left: Option<ParagraphBorder>,
    right: Option<ParagraphBorder>,
    top: Option<ParagraphBorder>,
    bottom: Option<ParagraphBorder>,
    between: Option<ParagraphBorder>,
    bar: Option<ParagraphBorder>,
}


impl Default for ParagraphBorders {
    fn default() -> Self {
        ParagraphBorders {
            left: Some(ParagraphBorder::new(ParagraphBorderPosition::Left)),
            right: Some(ParagraphBorder::new(ParagraphBorderPosition::Right)),
            top: Some(ParagraphBorder::new(ParagraphBorderPosition::Top)),
            bottom: Some(ParagraphBorder::new(ParagraphBorderPosition::Bottom)),
            between: None,
            bar: None,
        }
    }
}

impl ParagraphBorders {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_empty() -> Self {
        ParagraphBorders {
            left: None,
            right: None,
            top: None,
            bottom: None,
            between: None,
            bar: None,
        }
    }

    pub fn set(mut self, border: ParagraphBorder) -> Self {
        match border.position {
            ParagraphBorderPosition::Top => self.top = Some(border),
            ParagraphBorderPosition::Left => self.left = Some(border),
            ParagraphBorderPosition::Bottom => self.bottom = Some(border),
            ParagraphBorderPosition::Right => self.right = Some(border),
            ParagraphBorderPosition::Between => self.between = Some(border),
            ParagraphBorderPosition::Bar => self.bar = Some(border),
        };
        self
    }

    pub fn clear(mut self, position: ParagraphBorderPosition) -> Self {
        let nil = ParagraphBorder::new(position.clone()).val(BorderType::Nil);
        match position {
            ParagraphBorderPosition::Top => self.top = Some(nil),
            ParagraphBorderPosition::Left => self.left = Some(nil),
            ParagraphBorderPosition::Bottom => self.bottom = Some(nil),
            ParagraphBorderPosition::Right => self.right = Some(nil),
            ParagraphBorderPosition::Between => self.between = Some(nil),
            ParagraphBorderPosition::Bar => self.bar = Some(nil),
        };
        self
    }

    pub fn clear_all(mut self) -> Self {
        self.left = Some(ParagraphBorder::new(ParagraphBorderPosition::Left).val(BorderType::Nil));
        self.right = Some(ParagraphBorder::new(ParagraphBorderPosition::Right).val(BorderType::Nil));
        self.top = Some(ParagraphBorder::new(ParagraphBorderPosition::Top).val(BorderType::Nil));
        self.bottom = Some(ParagraphBorder::new(ParagraphBorderPosition::Bottom).val(BorderType::Nil));
        self.between = Some(ParagraphBorder::new(ParagraphBorderPosition::Between).val(BorderType::Nil));
        self.bar = Some(ParagraphBorder::new(ParagraphBorderPosition::Bar).val(BorderType::Nil));
        self
    }
}

impl BuildXML for ParagraphBorders {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_paragraph_borders()
            .add_optional_child(&self.left)
            .add_optional_child(&self.right)
            .add_optional_child(&self.top)
            .add_optional_child(&self.bottom)
            .add_optional_child(&self.between)
            .add_optional_child(&self.bar)
            .close()
            .build()
    }
}