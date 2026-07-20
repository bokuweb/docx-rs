use std::io::{Cursor, Read};

use super::ReaderError;

pub fn read_zip(
    archive: &mut zip::read::ZipArchive<Cursor<&[u8]>>,
    name: &str,
) -> Result<Vec<u8>, ReaderError> {
    // Archives zipped on Windows keep '\' in paths, replace them to avoid zip error.
    let normalized;
    let path = if name.contains('\\') {
        normalized = name.replace('\\', "/");
        normalized.trim_start_matches('/')
    } else {
        name.trim_start_matches('/')
    };
    let mut xml = archive.by_name(path)?;
    let capacity = usize::try_from(xml.size()).unwrap_or(0);
    let mut data = Vec::with_capacity(capacity);
    xml.read_to_end(&mut data).unwrap();
    // Remove BOM
    if data.starts_with(&[0xef, 0xbb, 0xbf]) {
        data.copy_within(3.., 0);
        data.truncate(data.len() - 3);
    }
    Ok(data)
}
