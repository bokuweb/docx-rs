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

#[test]
pub fn read_decoration() {
    let mut file = File::open("../fixtures/decoration/decoration.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_debug_snapshot!(&json);

    let path = std::path::Path::new("./tests/output/decoration.json");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_highlight_and_underline() {
    let mut file =
        File::open("../fixtures/highlight_and_underline/highlight_and_underline.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_debug_snapshot!(&json);

    let path = std::path::Path::new("./tests/output/highlight_and_underline.json");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_history() {
    let mut file = File::open("../fixtures/history_libre_office/history.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_debug_snapshot!(&json);

    let path = std::path::Path::new("./tests/output/history.json");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_indent_word_online() {
    let mut file = File::open("../fixtures/indent_word_online/indent.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_debug_snapshot!(&json);

    let path = std::path::Path::new("./tests/output/indent_word_online.json");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_tab_and_break() {
    let mut file = File::open("../fixtures/tab_and_break/tab_and_break.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_debug_snapshot!(&json);

    let path = std::path::Path::new("./tests/output/tab_and_break.json");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_table_docx() {
    let mut file = File::open("../fixtures/table_docx/table.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_debug_snapshot!(&json);

    let path = std::path::Path::new("./tests/output/table_docx.json");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_table_merged_libre_office() {
    let mut file = File::open("../fixtures/table_merged_libre_office/table_merged.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_debug_snapshot!(&json);

    let path = std::path::Path::new("./tests/output/table_merged_libre_office.json");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}
