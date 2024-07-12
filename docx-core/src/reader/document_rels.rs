use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::Cursor;
use std::path::*;

use document_rels::rels::find_rels_filename;
use document_rels::rels::read_rels_xml;

use super::errors::*;
use super::*;

pub type RId = String;

#[derive(Debug, Clone, PartialEq)]
pub struct ReadDocumentRels {
    rels: BTreeMap<String, BTreeSet<(RId, PathBuf, Option<String>)>>,
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
    Ok(ReadDocumentRels { rels })
}
