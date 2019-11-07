use super::Docx;
use crate::documents::BuildXML;

pub(crate) struct XMLDocument {
    content_type: Vec<u8>,
    rels: Vec<u8>,
}

impl From<Docx> for XMLDocument {
    fn from(doc: Docx) -> XMLDocument {
        let content_type = doc.content_type.build();
        XMLDocument {
            content_type,
            rels: vec![],
        }
    }
}
