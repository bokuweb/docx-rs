use super::Document;
use crate::documents::BuildXML;

pub(crate) struct XMLDocument {
    content_type: Vec<u8>,
    rels: Vec<u8>,
}

impl From<Document> for XMLDocument {
    fn from(doc: Document) -> XMLDocument {
        let content_type = doc.content_type.build();
        XMLDocument {
            content_type,
            rels: vec![],
        }
    }
}
