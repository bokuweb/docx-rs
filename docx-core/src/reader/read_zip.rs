use std::io::{Cursor, Read};

use super::ReaderError;

pub fn read_zip(
    archive: &mut zip::read::ZipArchive<Cursor<&[u8]>>,
    name: &str,
) -> Result<Vec<u8>, ReaderError> {
    let p = name.to_owned();
    // Archives zipped on Windows keep '\' in paths, replace them to avoid zip error.
    let mut p = str::replace(&p, "\\", "/");
    if p.starts_with('/') {
        p.remove(0);
    }
    let mut xml = archive.by_name(&p)?;
    let mut data = vec![];
    xml.read_to_end(&mut data).unwrap();
    // Remove BOM
    if (data[0] == 0xef) && (data[1] == 0xbb) && (data[2] == 0xbf) {
        data.remove(0);
        data.remove(0);
        data.remove(0);
    }
    Ok(data)
}
