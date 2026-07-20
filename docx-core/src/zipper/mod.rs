use crate::xml_builder::XMLBuilder;
use crate::{BuildXML, DocumentChild, Docx, Footer, Header, PackageMetadata, XMLDocx};

use std::io::prelude::*;
use std::io::Seek;
use zip::write::SimpleFileOptions;

/// Writes one XML part directly into the current ZIP entry.
fn write_xml<W, T>(
    zip: &mut zip::ZipWriter<W>,
    path: &str,
    options: SimpleFileOptions,
    part: &T,
) -> zip::result::ZipResult<()>
where
    W: Write + Seek,
    T: BuildXML,
{
    zip.start_file(path, options)?;
    let stream = XMLBuilder::new(&mut *zip)
        .into_inner()
        .map_err(xml_to_zip_error)?;
    let stream = part.build_to(stream).map_err(xml_to_zip_error)?;
    stream.into_inner().map_err(xml_to_zip_error)?;
    Ok(())
}

/// Converts an XML writer failure into the archive writer's error type.
fn xml_to_zip_error(error: crate::xml::writer::Error) -> zip::result::ZipError {
    zip::result::ZipError::Io(std::io::Error::other(error))
}

pub fn zip<W>(w: W, xml: XMLDocx) -> zip::result::ZipResult<()>
where
    W: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(w);

    let directory_options = SimpleFileOptions::default();

    zip.add_directory("word/", directory_options)?;
    zip.add_directory("word/_rels", directory_options)?;
    zip.add_directory("_rels/", directory_options)?;
    zip.add_directory("docProps/", directory_options)?;

    let options = SimpleFileOptions::default()
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
        zip.add_directory("word/media/", directory_options)?;
        for m in xml.media {
            // For now only png supported
            zip.start_file(format!("word/media/{}.png", m.0), options)?;
            zip.write_all(&m.1)?;
        }
    }

    // For now support only taskpanes
    if let Some(taskpanes) = xml.taskpanes {
        zip.add_directory("word/webextensions/", directory_options)?;
        zip.start_file("word/webextensions/taskpanes.xml", options)?;
        zip.write_all(&taskpanes)?;

        zip.add_directory("word/webextensions/_rels", directory_options)?;
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
        zip.add_directory("customXml/_rels", directory_options)?;
    }

    for (i, item) in xml.custom_items.into_iter().enumerate() {
        let n = i + 1;
        zip.start_file(format!("customXml/_rels/item{n}.xml.rels"), options)?;
        zip.write_all(&xml.custom_item_rels[i])?;
        zip.start_file(format!("customXml/item{n}.xml"), options)?;
        zip.write_all(&item)?;
        zip.start_file(format!("customXml/itemProps{n}.xml"), options)?;
        zip.write_all(&xml.custom_item_props[i])?;
    }

    zip.finish()?;
    Ok(())
}

/// Streams a prepared document into a DOCX archive without buffering every
/// XML package part in memory.
pub(crate) fn zip_docx<W>(
    writer: W,
    docx: Docx,
    package: PackageMetadata,
) -> zip::result::ZipResult<()>
where
    W: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let directory_options = SimpleFileOptions::default();

    zip.add_directory("word/", directory_options)?;
    zip.add_directory("word/_rels", directory_options)?;
    zip.add_directory("_rels/", directory_options)?;
    zip.add_directory("docProps/", directory_options)?;

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    write_xml(&mut zip, "[Content_Types].xml", options, &docx.content_type)?;
    write_xml(&mut zip, "_rels/.rels", options, &docx.rels)?;
    write_xml(&mut zip, "docProps/app.xml", options, &docx.doc_props.app)?;
    write_xml(&mut zip, "docProps/core.xml", options, &docx.doc_props.core)?;
    write_xml(
        &mut zip,
        "docProps/custom.xml",
        options,
        &docx.doc_props.custom,
    )?;
    write_xml(
        &mut zip,
        "word/_rels/document.xml.rels",
        options,
        &docx.document_rels,
    )?;
    write_xml(&mut zip, "word/document.xml", options, &docx.document)?;
    write_xml(&mut zip, "word/styles.xml", options, &docx.styles)?;
    write_xml(&mut zip, "word/settings.xml", options, &docx.settings)?;
    write_xml(&mut zip, "word/fontTable.xml", options, &docx.font_table)?;
    write_xml(&mut zip, "word/comments.xml", options, &docx.comments)?;
    write_xml(&mut zip, "word/numbering.xml", options, &docx.numberings)?;
    write_xml(
        &mut zip,
        "word/commentsExtended.xml",
        options,
        &docx.comments_extended,
    )?;
    write_xml(&mut zip, "word/footnotes.xml", options, &docx.footnotes)?;

    let mut headers: Vec<&(String, Header)> = docx.document.section_property.get_headers();
    for child in &docx.document.children {
        if let DocumentChild::Section(section) = child {
            headers.extend(section.property.get_headers());
        }
    }
    headers.sort_by(|left, right| left.0.cmp(&right.0));
    for (index, (_, header)) in headers.into_iter().enumerate() {
        let number = index + 1;
        write_xml(
            &mut zip,
            &format!("word/header{number}.xml"),
            options,
            header,
        )?;
        if let Some(rels) = package.header_rels.get(index) {
            write_xml(
                &mut zip,
                &format!("word/_rels/header{number}.xml.rels"),
                options,
                rels,
            )?;
        }
    }

    let mut footers: Vec<&(String, Footer)> = docx.document.section_property.get_footers();
    for child in &docx.document.children {
        if let DocumentChild::Section(section) = child {
            footers.extend(section.property.get_footers());
        }
    }
    footers.sort_by(|left, right| left.0.cmp(&right.0));
    for (index, (_, footer)) in footers.into_iter().enumerate() {
        let number = index + 1;
        write_xml(
            &mut zip,
            &format!("word/footer{number}.xml"),
            options,
            footer,
        )?;
        if let Some(rels) = package.footer_rels.get(index) {
            write_xml(
                &mut zip,
                &format!("word/_rels/footer{number}.xml.rels"),
                options,
                rels,
            )?;
        }
    }

    if !package.media.is_empty() {
        zip.add_directory("word/media/", directory_options)?;
        for (id, bytes) in package.media {
            zip.start_file(format!("word/media/{id}.png"), options)?;
            zip.write_all(&bytes)?;
        }
    }

    if let Some(taskpanes) = &docx.taskpanes {
        zip.add_directory("word/webextensions/", directory_options)?;
        write_xml(
            &mut zip,
            "word/webextensions/taskpanes.xml",
            options,
            taskpanes,
        )?;

        zip.add_directory("word/webextensions/_rels", directory_options)?;
        write_xml(
            &mut zip,
            "word/webextensions/_rels/taskpanes.xml.rels",
            options,
            &docx.taskpanes_rels,
        )?;

        for (index, extension) in docx.web_extensions.iter().enumerate() {
            write_xml(
                &mut zip,
                &format!("word/webextensions/webextension{}.xml", index + 1),
                options,
                extension,
            )?;
        }
    }

    if !docx.custom_items.is_empty() {
        zip.add_directory("customXml/_rels", directory_options)?;
    }
    for (index, item) in docx.custom_items.iter().enumerate() {
        let number = index + 1;
        write_xml(
            &mut zip,
            &format!("customXml/_rels/item{number}.xml.rels"),
            options,
            &docx.custom_item_rels[index],
        )?;
        write_xml(
            &mut zip,
            &format!("customXml/item{number}.xml"),
            options,
            item,
        )?;
        write_xml(
            &mut zip,
            &format!("customXml/itemProps{number}.xml"),
            options,
            &docx.custom_item_props[index],
        )?;
    }

    zip.finish()?;
    Ok(())
}
