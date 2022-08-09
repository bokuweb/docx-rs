use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io::{Cursor, Read};
use std::path::*;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};

use super::errors::*;
use super::*;

pub type RId = String;

#[derive(Debug, Clone, PartialEq)]
pub struct ReadDocumentRels {
    rels: BTreeMap<String, HashSet<(RId, PathBuf, Option<String>)>>,
}

impl ReadDocumentRels {
    pub fn find_target_path(&self, target: &str) -> Option<Vec<(RId, PathBuf, Option<String>)>> {
        self.rels
            .get(target)
            .map(|s| s.clone().into_iter().collect())
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
    let data = read_zip(archive, p)?;
    let rels = read_rels_xml(&data[..], dir)?;
    Ok(rels)
}

fn read_rels_xml<R: Read>(
    reader: R,
    dir: impl AsRef<Path>,
) -> Result<ReadDocumentRels, ReaderError> {
    let mut parser = EventReader::new(reader);
    let mut rels = ReadDocumentRels {
        rels: BTreeMap::new(),
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
                    let mut rid = "".to_owned();
                    let mut target_mode = None;
                    let mut target_string = "".to_owned();
                    for a in attributes {
                        let local_name = &a.name.local_name;
                        if local_name == "Type" {
                            rel_type = a.value.to_owned();
                        } else if local_name == "Target" {
                            // target_str = Path::new(dir.as_ref()).join(a.value);
                            target_string = a.value.to_owned();
                        } else if local_name == "Id" {
                            rid = a.value.to_owned();
                        } else if local_name == "TargetMode" {
                            target_mode = Some(a.value.to_owned());
                        }
                    }

                    let target = if !rel_type.ends_with("hyperlink") {
                        Path::new(dir.as_ref()).join(target_string)
                    } else {
                        Path::new("").join(target_string)
                    };

                    let current = rels.rels.remove(&rel_type);
                    if let Some(mut paths) = current {
                        paths.insert((rid, target, target_mode));
                        rels.rels.insert(rel_type, paths);
                    } else {
                        let s: HashSet<(RId, PathBuf, Option<String>)> =
                            vec![(rid, target, target_mode)].into_iter().collect();
                        rels.rels.insert(rel_type, s);
                    }
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
