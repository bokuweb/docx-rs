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

/// OPC ZIP item names are ASCII (ECMA-376 Part 2, 7.3.3). An extra-field
/// override must not silently redirect an ASCII relationship to another part.
pub(crate) fn validate_ascii_part_names(
    bytes: &[u8],
    archive: &mut zip::read::ZipArchive<Cursor<&[u8]>>,
) -> Result<(), ReaderError> {
    let invalid_header =
        || zip::result::ZipError::InvalidArchive("Invalid ZIP filename header".into());
    for index in 0..archive.len() {
        let entry = archive.by_index_raw(index)?;
        for (start, length_offset, name_offset) in [
            (entry.header_start(), 26, 30),
            (entry.central_header_start(), 28, 46),
        ] {
            let start = usize::try_from(start).map_err(|_| invalid_header())?;
            let header = bytes.get(start..).ok_or_else(invalid_header)?;
            let length = header
                .get(length_offset..length_offset + 2)
                .ok_or_else(invalid_header)?;
            let name_length = usize::from(u16::from_le_bytes([length[0], length[1]]));
            let raw_name = header
                .get(name_offset..name_offset + name_length)
                .ok_or_else(invalid_header)?;
            if raw_name.is_ascii() && raw_name != entry.name().as_bytes() {
                return Err(zip::result::ZipError::InvalidArchive(
                    "ASCII ZIP part name disagrees with resolved filename".into(),
                )
                .into());
            }
        }
    }
    Ok(())
}
