use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Zoom {
    val: usize,
}

impl Zoom {
    pub fn new(val: usize) -> Zoom {
        Zoom { val }
    }
}

impl BuildXML for Zoom {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.zoom(&format!("{}", self.val)).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_zoom() {
        let c = Zoom::new(20);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:zoom w:percent="20" />"#);
    }
}
