use super::{Color, Sz, SzCs};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct RunProperty {
    sz: Option<Sz>,
    sz_cs: Option<SzCs>,
    color: Option<Color>,
}

impl RunProperty {
    pub fn new() -> RunProperty {
        Default::default()
    }

    pub fn size(mut self, size: usize) -> RunProperty {
        self.sz = Some(Sz::new(size));
        self.sz_cs = Some(SzCs::new(size));
        self
    }

    pub fn color(mut self, color: &str) -> RunProperty {
        self.color = Some(Color::new(color));
        self
    }
}

impl Default for RunProperty {
    fn default() -> Self {
        Self {
            color: None,
            sz: None,
            sz_cs: None,
        }
    }
}

impl BuildXML for RunProperty {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_run_property()
            .add_optional_child(&self.sz)
            .add_optional_child(&self.sz_cs)
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
        let c = RunProperty::new().size(10).color("FFFFFF");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:sz w:val="10" /><w:szCs w:val="10" /><w:color w:val="FFFFFF" /></w:rPr>"#
        );
    }
}
