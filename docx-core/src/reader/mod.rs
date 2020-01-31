mod errors;

use errors::*;
use serde_json;
use std::fs;
use std::io::Cursor;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};
use zip;

use crate::documents::ContentTypes;
use crate::documents::FromXML;

pub use errors::ReaderError;

pub fn read_docx(buf: &[u8]) -> Result<(), ReaderError> {
    let cur = Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(cur)?;

    // First, the content type for relationship parts and the Main Document part
    // (the only required part) must be defined (physically located at /[Content_Types].xml in the package)
    let content_types = archive.by_name("[Content_Types].xml")?;
    let c = ContentTypes::from_xml(content_types)?;
    let serialized = serde_json::to_string(&c).unwrap();

    println!("serialized = {}", serialized);

    /*
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        let path = file.sanitized_name();
        // Skip directory
        if (&*file.name()).ends_with('/') {
            continue;
        }
        println!(
            "File {} extracted to \"{}\" ({} bytes)",
            i,
            path.as_path().display(),
            file.size()
        );
        let parser = EventReader::new(file);
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    println!("{}+{}", depth, name);
                    dbg!(attributes);
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    // println!("{}-{}", depth, name);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }
    */
    Ok(())
}
