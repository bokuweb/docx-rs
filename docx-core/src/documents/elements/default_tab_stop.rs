use crate::documents::BuildXML;
use crate::xml_builder::*;
use std::io::Write;

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultTabStop {
    val: usize,
}

impl DefaultTabStop {
    pub fn new(val: usize) -> DefaultTabStop {
        DefaultTabStop { val }
    }
}

impl BuildXML for DefaultTabStop {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .default_tab_stop(self.val)?
            .into_inner()
    }
}

impl Serialize for DefaultTabStop {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.val as u64)
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
        let c = DefaultTabStop::new(20);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:defaultTabStop w:val="20" />"#
        );
    }
}
