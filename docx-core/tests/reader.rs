use insta::assert_debug_snapshot;

use docx_rs::*;
use std::fs::*;
use std::io::{Read, Write};

#[test]
pub fn read_hello() {
    let mut file = File::open("../fixtures/hello_world/hello_world.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_debug_snapshot!(&json);

    let path = std::path::Path::new("./tests/output/hello.json");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_numbering() {
    let mut file = File::open("../fixtures/numbering/numbering.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_debug_snapshot!(&json);

    let path = std::path::Path::new("./tests/output/numbering.json");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}
