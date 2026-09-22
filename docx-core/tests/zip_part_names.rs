use docx_rs::{read_docx, Docx, Paragraph, Run};
use std::io::{Cursor, Read, Write};
use zip::write::FullFileOptions;

fn document_with_unicode_names(overrides: &[(&str, &str)]) -> Vec<u8> {
    let mut original = Cursor::new(Vec::new());
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Original document")))
        .build()
        .pack(&mut original)
        .unwrap();
    let mut source = zip::ZipArchive::new(Cursor::new(original.into_inner())).unwrap();
    let mut parts = Vec::new();
    for index in 0..source.len() {
        let mut entry = source.by_index(index).unwrap();
        let mut contents = Vec::new();
        entry.read_to_end(&mut contents).unwrap();
        parts.push((entry.name().to_owned(), contents));
    }
    let document = parts
        .iter()
        .find(|(name, _)| name == "word/document.xml")
        .unwrap();
    let alternate = String::from_utf8(document.1.clone())
        .unwrap()
        .replace("Original document", "Alternate document");
    parts.push(("word/alternate.xml".to_owned(), alternate.into_bytes()));
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (name, contents) in parts {
        let mut options = FullFileOptions::default();
        if let Some((_, unicode_name)) = overrides.iter().find(|(raw, _)| *raw == name) {
            let mut extra = vec![1];
            extra.extend_from_slice(&crc32fast::hash(name.as_bytes()).to_le_bytes());
            extra.extend_from_slice(unicode_name.as_bytes());
            options.add_extra_data(0xfffe, extra, false).unwrap();
        }
        writer.start_file(name, options).unwrap();
        writer.write_all(&contents).unwrap();
    }
    let mut bytes = writer.finish().unwrap().into_inner();
    let mut archive = zip::ZipArchive::new(Cursor::new(&bytes)).unwrap();
    let headers: Vec<(usize, usize, usize)> = (0..archive.len())
        .flat_map(|index| {
            let entry = archive.by_index_raw(index).unwrap();
            [
                (entry.header_start() as usize, 26, 30),
                (entry.central_header_start() as usize, 28, 46),
            ]
        })
        .collect();
    drop(archive);
    // The writer validates Unicode fields before it has a filename. Write a
    // private extra field first, then give that field its Unicode Path ID.
    for (start, length_offset, name_offset) in headers {
        let name_len = u16::from_le_bytes(
            bytes[start + length_offset..start + length_offset + 2]
                .try_into()
                .unwrap(),
        ) as usize;
        let extra_len = u16::from_le_bytes(
            bytes[start + length_offset + 2..start + length_offset + 4]
                .try_into()
                .unwrap(),
        ) as usize;
        let mut offset = start + name_offset + name_len;
        let end = offset + extra_len;
        while offset + 4 <= end {
            let length =
                u16::from_le_bytes(bytes[offset + 2..offset + 4].try_into().unwrap()) as usize;
            if bytes[offset..offset + 2] == [0xfe, 0xff] {
                bytes[offset..offset + 2].copy_from_slice(&0x7075u16.to_le_bytes());
            }
            offset += 4 + length;
        }
    }
    bytes
}

#[test]
fn conflicting_unicode_part_names_are_rejected() {
    let bytes = document_with_unicode_names(&[
        ("word/document.xml", "word/alternate.xml"),
        ("word/alternate.xml", "word/document.xml"),
    ]);
    assert!(
        read_docx(&bytes).is_err(),
        "ambiguous names must not select the alternate document"
    );
}

#[test]
fn matching_unicode_part_names_preserve_the_document() {
    let bytes = document_with_unicode_names(&[("word/document.xml", "word/document.xml")]);
    let document = read_docx(&bytes).unwrap().json();
    assert!(document.contains("Original document"));
    assert!(!document.contains("Alternate document"));
}
