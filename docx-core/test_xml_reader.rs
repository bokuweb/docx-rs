use std::fs;

// Import from the local crate
extern crate docx_rs;
use docx_rs::read_xml;

fn main() {
    // Read the provided XML file
    let xml_content =
        fs::read_to_string("/Users/bokuweb/Downloads/ooxml.xml").expect("Failed to read XML file");

    println!(
        "Reading XML file with length: {} characters",
        xml_content.len()
    );

    match read_xml(&xml_content) {
        Ok(docx) => {
            println!("Successfully parsed XML!");
            println!("Document has {} children", docx.document.children.len());
            println!("Themes: {}", docx.themes.len());
            println!("Styles: {} styles", docx.styles.styles.len());
        }
        Err(e) => {
            println!("Failed to parse XML: {:?}", e);
        }
    }
}
