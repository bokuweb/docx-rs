use super::{Color, Sz};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct RunProperty {
    sz: Option<Sz>,
    color: Option<Color>,
}

impl RunProperty {
    pub fn new() -> RunProperty {
        Default::default()
    }

    pub fn add_sz(mut self, sz: usize) -> RunProperty {
        self.sz = Some(Sz::new(sz));
        self
    }

    pub fn add_color(mut self, color: &str) -> RunProperty {
        self.color = Some(Color::new(color));
        self
    }
}

impl Default for RunProperty {
    fn default() -> Self {
        Self {
            sz: None,
            color: None,
        }
    }
}

impl BuildXML for RunProperty {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_run_property()
            .add_optional_child(&self.sz)
            .add_optional_child(&self.color)
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
    fn test_build() {
        let c = RunProperty::new().add_sz(10).add_color("FFFFFF");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:sz w:val="10" /><w:color w:val="FFFFFF" /></w:rPr>"#
        );
    }
}
