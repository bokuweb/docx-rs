use crate::XMLDocx;

use std::io::prelude::*;
use zip::write::FileOptions;

pub fn zip(filename: &str, xml: XMLDocx) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(filename);
    let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    zip.add_directory("word/", Default::default())?;
    zip.add_directory("word/_rels", Default::default())?;
    zip.add_directory("_rels/", Default::default())?;
    zip.add_directory("docProps/", Default::default())?;

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    zip.start_file("[Content_Types].xml", options)?;
    zip.write_all(&xml.content_type)?;
    zip.start_file("_rels/.rels", options)?;
    zip.write_all(&xml.rels)?;
    zip.start_file("docProps/app.xml", options)?;
    zip.write_all(&xml.doc_props.app)?;
    zip.start_file("docProps/core.xml", options)?;
    zip.write_all(&xml.doc_props.core)?;
    zip.start_file("word/_rels/document.xml.rels", options)?;
    zip.write_all(&xml.rels)?;
    zip.start_file("word/document.xml", options)?;
    zip.write_all(&xml.document)?;
    zip.start_file("word/styles.xml", options)?;
    zip.write_all(&xml.styles)?;
    zip.finish()?;
    Ok(())
}
