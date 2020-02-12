use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::*;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};

use super::errors::*;
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ReadDocumentRels {
    rels: HashMap<String, PathBuf>,
}

impl ReadDocumentRels {
    pub fn find_target_path(&self, target: &str) -> Option<PathBuf> {
        self.rels.get(target).cloned()
    }
}

pub fn read_document_rels(
    archive: &mut zip::read::ZipArchive<Cursor<&[u8]>>,
    main_path: impl AsRef<Path>,
) -> Result<ReadDocumentRels, ReaderError> {
    let dir = &main_path
        .as_ref()
        .parent()
        .ok_or(ReaderError::DocumentRelsNotFoundError)?;
    let p = find_rels_filename(&main_path)?;
    let p = p.to_str().ok_or(ReaderError::DocumentRelsNotFoundError)?;
    let rels_xml = archive.by_name(&p)?;
    let rels = read_rels_xml(rels_xml, dir)?;
    Ok(rels)
}

fn read_rels_xml<R: Read>(
    reader: R,
    dir: impl AsRef<Path>,
) -> Result<ReadDocumentRels, ReaderError> {
    let mut parser = EventReader::new(reader);
    let mut rels = ReadDocumentRels {
        rels: HashMap::new(),
    };
    loop {
        let e = parser.next();
        match e {
            Ok(XmlEvent::StartElement {
                attributes, name, ..
            }) => {
                let e = XMLElement::from_str(&name.local_name).unwrap();
                if let XMLElement::Relationship = e {
                    let mut rel_type = "".to_owned();
                    let mut target = PathBuf::default();
                    for a in attributes {
                        let local_name = &a.name.local_name;
                        if local_name == "Type" {
                            rel_type = a.value.to_owned();
                        } else if local_name == "Target" {
                            target = Path::new(dir.as_ref()).join(a.value);
                        }
                    }
                    rels.rels.insert(rel_type, target);
                    continue;
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                let e = XMLElement::from_str(&name.local_name).unwrap();
                if let XMLElement::Relationships = e {
                    break;
                }
            }
            Err(_) => return Err(ReaderError::XMLReadError),
            _ => {}
        }
    }
    Ok(rels)
}

fn find_rels_filename(main_path: impl AsRef<Path>) -> Result<PathBuf, ReaderError> {
    let path = main_path.as_ref();
    let dir = path
        .parent()
        .ok_or(ReaderError::DocumentRelsNotFoundError)?;
    let base = path
        .file_stem()
        .ok_or(ReaderError::DocumentRelsNotFoundError)?;
    Ok(Path::new(dir)
        .join("_rels")
        .join(base)
        .with_extension("xml.rels"))
}
