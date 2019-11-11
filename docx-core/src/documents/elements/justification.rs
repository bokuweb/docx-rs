use crate::documents::BuildXML;
use crate::xml_builder::*;

// 22.1.2.51
// jc (Justification)
// This element specifies justification of the math paragraph (a series of adjacent instances of mathematical text
// within the same paragraph). A math paragraph can be Left Justified, Right Justified, Centered, or Centered as
// Group. If this element is omitted, the math paragraph is Centered as Group. Whether the element is absent or
// present without the val attribute, the default of the val attribute is centerGroup . This means that the instances
// of mathematical text can be aligned with respect to each other, but the entire group of mathematical text is
// centered as a whole.
#[derive(Debug)]
pub struct Justification {
    val: String,
}

impl Justification {
    pub fn new(val: impl Into<String>) -> Justification {
        Justification { val: val.into() }
    }
}

impl BuildXML for Justification {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.justification(&self.val).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_justification() {
        let c = Justification::new("start");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:jc w:val="start" />"#);
    }
}
