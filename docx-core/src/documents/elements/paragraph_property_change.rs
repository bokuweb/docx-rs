use serde::Serialize;

use crate::documents::*;
use crate::escape;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphPropertyChange {
    pub author: String,
    pub date: String,
    pub property: Box<ParagraphProperty>,
}

impl Default for ParagraphPropertyChange {
    fn default() -> ParagraphPropertyChange {
        Self {
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            property: Box::new(ParagraphProperty::default()),
        }
    }
}

impl ParagraphPropertyChange {
    pub fn new() -> ParagraphPropertyChange {
        Self {
            ..Default::default()
        }
    }

    pub fn property(mut self, p: ParagraphProperty) -> ParagraphPropertyChange {
        self.property = Box::new(p);
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> ParagraphPropertyChange {
        self.author = escape::escape(&author.into());
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> ParagraphPropertyChange {
        self.date = date.into();
        self
    }
}

impl ParagraphPropertyChangeId for ParagraphPropertyChange {}

impl BuildXML for ParagraphPropertyChange {
    #[allow(clippy::needless_borrow)]
    fn build(&self) -> Vec<u8> {
        let id = self.generate();
        XMLBuilder::new()
            .open_paragraph_property_change(&id, &self.author, &self.date)
            .add_child(&self.property)
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
    fn test_ppr_change_default() {
        let b = ParagraphPropertyChange::new()
            .property(ParagraphProperty::new())
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPrChange w:id="123" w:author="unnamed" w:date="1970-01-01T00:00:00Z"><w:pPr><w:rPr /></w:pPr></w:pPrChange>"#
        );
    }
}
