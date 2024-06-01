use crate::XMLDocx;

use std::io::prelude::*;
use std::io::Seek;
use zip::write::FileOptions;

pub fn zip<W>(w: W, xml: XMLDocx) -> zip::result::ZipResult<()>
where
    W: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(w);

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
    zip.start_file("docProps/custom.xml", options)?;
    zip.write_all(&xml.doc_props.custom)?;
    zip.start_file("word/_rels/document.xml.rels", options)?;
    zip.write_all(&xml.document_rels)?;
    zip.start_file("word/document.xml", options)?;
    zip.write_all(&xml.document)?;
    zip.start_file("word/styles.xml", options)?;
    zip.write_all(&xml.styles)?;
    zip.start_file("word/settings.xml", options)?;
    zip.write_all(&xml.settings)?;
    zip.start_file("word/fontTable.xml", options)?;
    zip.write_all(&xml.font_table)?;
    zip.start_file("word/comments.xml", options)?;
    zip.write_all(&xml.comments)?;
    zip.start_file("word/numbering.xml", options)?;
    zip.write_all(&xml.numberings)?;
    zip.start_file("word/commentsExtended.xml", options)?;
    zip.write_all(&xml.comments_extended)?;
    zip.start_file("word/footnotes.xml", options)?;
    zip.write_all(&xml.footnotes)?;

    for (i, h) in xml.headers.iter().enumerate() {
        zip.start_file(format!("word/header{}.xml", i + 1), options)?;
        zip.write_all(h)?;

        if let Some(rels) = xml.header_rels.get(i) {
            zip.start_file(format!("word/_rels/header{}.xml.rels", i + 1), options)?;
            zip.write_all(rels)?;
        }
    }

    for (i, h) in xml.footers.iter().enumerate() {
        zip.start_file(format!("word/footer{}.xml", i + 1), options)?;
        zip.write_all(h)?;

        if let Some(rels) = xml.footer_rels.get(i) {
            zip.start_file(format!("word/_rels/footer{}.xml.rels", i + 1), options)?;
            zip.write_all(rels)?;
        }
    }

    if !xml.media.is_empty() {
        zip.add_directory("word/media/", Default::default())?;
        for m in xml.media {
            // For now only png supported
            zip.start_file(format!("word/media/{}.png", m.0), options)?;
            zip.write_all(&m.1)?;
        }
    }

    // For now support only taskpanes
    if let Some(taskpanes) = xml.taskpanes {
        zip.add_directory("word/webextensions/", Default::default())?;
        zip.start_file("word/webextensions/taskpanes.xml", options)?;
        zip.write_all(&taskpanes)?;

        zip.add_directory("word/webextensions/_rels", Default::default())?;
        zip.start_file("word/webextensions/_rels/taskpanes.xml.rels", options)?;
        zip.write_all(&xml.taskpanes_rels)?;

        for (i, ext) in xml.web_extensions.iter().enumerate() {
            zip.start_file(
                format!("word/webextensions/webextension{}.xml", i + 1),
                options,
            )?;
            zip.write_all(ext)?;
        }
    }

    if !xml.custom_items.is_empty() {
        zip.add_directory("customXml/_rels", Default::default())?;
    }

    for (i, item) in xml.custom_items.into_iter().enumerate() {
        let n = i + 1;
        zip.start_file(format!("customXml/_rels/item{}.xml.rels", n), options)?;
        zip.write_all(&xml.custom_item_rels[i])?;
        zip.start_file(format!("customXml/item{}.xml", n), options)?;
        zip.write_all(&item)?;
        zip.start_file(format!("customXml/itemProps{}.xml", n), options)?;
        zip.write_all(&xml.custom_item_props[i])?;
    }

    zip.finish()?;
    Ok(())
}
