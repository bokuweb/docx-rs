use super::*;
use crate::reader::{FromXML, ReaderError};
use std::str::FromStr;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::{Path, PathBuf},
};
use xml::reader::{EventReader, XmlEvent};

pub type ReadRels = BTreeMap<String, BTreeSet<(RId, PathBuf, Option<String>)>>;

impl FromXML for Rels {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let parser = EventReader::new(reader);
        let mut s = Self::default();
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { attributes, .. }) => {
                    if depth == 1 {
                        let mut rel_type = "".to_owned();
                        let mut target = "".to_owned();
                        for attr in attributes {
                            let name: &str = &attr.name.local_name;
                            if name == "Type" {
                                rel_type = attr.value.clone();
                            } else if name == "Target" {
                                target = attr.value.clone();
                            }
                        }
                        s = s.add_rel(rel_type, target);
                    }
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    depth -= 1;
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
        Ok(s)
    }
}

pub fn find_rels_filename(main_path: impl AsRef<Path>) -> Result<PathBuf, ReaderError> {
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

pub fn read_rels_xml<R: Read>(reader: R, dir: impl AsRef<Path>) -> Result<ReadRels, ReaderError> {
    let mut parser = EventReader::new(reader);
    let mut rels: BTreeMap<String, BTreeSet<(RId, PathBuf, Option<String>)>> = BTreeMap::new();

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

                    let current = rels.remove(&rel_type);
                    if let Some(mut paths) = current {
                        paths.insert((rid, target, target_mode));
                        rels.insert(rel_type, paths);
                    } else {
                        let s: BTreeSet<(RId, PathBuf, Option<String>)> =
                            vec![(rid, target, target_mode)].into_iter().collect();
                        rels.insert(rel_type, s);
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

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml" />
</Relationships>"#;
        let c = Rels::from_xml(xml.as_bytes()).unwrap();
        let rels =
            vec![
        (
            "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
                .to_owned(),
            "rId1".to_owned(),
            "docProps/core.xml".to_owned(),
        )];
        assert_eq!(Rels { rels }, c);
    }
}
