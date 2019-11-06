use super::Sz;
use crate::documents::BuildXML;
use crate::xml_builder::*;

pub struct RunProperty {
    sz: Option<Sz>,
}

impl RunProperty {
    pub fn new() -> RunProperty {
        RunProperty { sz: None }
    }

    pub fn add_sz(mut self, sz: Sz) -> RunProperty {
        self.sz = Some(sz);
        self
    }
}

impl BuildXML for RunProperty {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let b = b.open_run_property();
        let b = if let Some(sz) = &self.sz {
            b.add_child_buffer(&sz.build())
        } else {
            b
        };
        b.close().build()
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
        let c = RunProperty::new().add_sz(Sz::new(10));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPr><w:sz w:val="10" /></w:rPr>"#
        );
    }
}
