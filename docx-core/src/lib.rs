mod documents;
mod xml_builder;

use documents::*;
use xml_builder::*;

use std::fs::File;
use std::io::{self, Write};

use xml::writer::{EmitterConfig, EventWriter, Result, XmlEvent};

pub fn simple() {
    let doc = Document::new();
    let mut file = File::create("./dist/output.xml").unwrap();
    // let mut b = Vec::new();
    // let mut w = EmitterConfig::new()
    //     .write_document_declaration(false)
    //     .create_writer(&mut b);
    // w.write(
    //     XmlEvent::start_element("?xml")
    //         .attr("version", "1.0")
    //         .attr("encoding", "UTF-8"),
    // );
    // // w.write("hello world").unwrap();
    // w.write(XmlEvent::end_element()).unwrap();
    // file.write_all(&b).unwrap();
    // file.flush().unwrap();
    // assert_eq!(
    //     str::from_utf8(&b).unwrap(),
    //     r#"<h:hello xmlns:h="urn:hello-world">hello world</h:hello>"#
    // );
}
