use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io::Cursor;
use std::path::*;

use header_or_footer_rels::rels::find_rels_filename;

use self::rels::read_rels_xml;

use super::errors::*;
use super::*;

pub type RId = String;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ReadHeaderOrFooterRels {
    pub rels: BTreeMap<String, HashSet<(RId, PathBuf, Option<String>)>>,
}

impl ReadHeaderOrFooterRels {
    pub fn find_target_path(&self, target: &str) -> Option<Vec<(RId, PathBuf, Option<String>)>> {
        self.rels
            .get(target)
            .map(|s| s.clone().into_iter().collect())
    }
}

pub fn read_header_or_footer_rels(
    archive: &mut zip::read::ZipArchive<Cursor<&[u8]>>,
    header_or_footer_path: impl AsRef<Path>,
) -> Result<ReadHeaderOrFooterRels, ReaderError> {
    let dir = &header_or_footer_path
        .as_ref()
        .parent()
        .ok_or(ReaderError::HeaderOrFooterRelsNotFoundError)?;
    let p = find_rels_filename(&header_or_footer_path)?;
    let p = p
        .to_str()
        .ok_or(ReaderError::HeaderOrFooterRelsNotFoundError)?;
    let data = read_zip(archive, p)?;
    let rels = read_rels_xml(&data[..], dir)?;
    Ok(ReadHeaderOrFooterRels { rels })
}
