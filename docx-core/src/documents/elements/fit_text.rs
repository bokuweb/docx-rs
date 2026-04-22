use crate::documents::BuildXML;
use crate::xml_builder::*;
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FitText {
    pub val: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

impl FitText {
    pub fn new(val: usize) -> Self {
        Self { val, id: None }
    }

    pub fn id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }
}

impl BuildXML for FitText {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .fit_text(self.val, self.id)?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_fit_text() {
        let b = FitText::new(840).id(1266434317).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:fitText w:val="840" w:id="1266434317" />"#
        );
    }

    #[test]
    fn test_fit_text_without_id() {
        let b = FitText::new(840).build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:fitText w:val="840" />"#);
    }
}
