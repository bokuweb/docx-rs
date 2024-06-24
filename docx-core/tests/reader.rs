use insta::assert_json_snapshot;

use docx_rs::*;
use std::fs::*;
use std::io::{Read, Write};

#[test]
pub fn read_hello() {
    let mut file = File::open("../fixtures/hello_world/hello_world.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_hello", &json);

    let path = std::path::Path::new("./tests/output/hello.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_numbering() {
    let mut file = File::open("../fixtures/numbering/numbering.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_numbering", &json);

    let path = std::path::Path::new("./tests/output/numbering.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_decoration() {
    let mut file = File::open("../fixtures/decoration/decoration.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_decoration", &json);

    let path = std::path::Path::new("./tests/output/decoration.json");
    let mut file = std::fs::File::create(path).unwrap();
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

    assert_json_snapshot!("read_highlight_and_underline", &json);

    let path = std::path::Path::new("./tests/output/highlight_and_underline.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_history() {
    let mut file = File::open("../fixtures/history_libre_office/history.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_history", &json);

    let path = std::path::Path::new("./tests/output/history.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_indent_word_online() {
    let mut file = File::open("../fixtures/indent_word_online/indent.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_indent_word_online", &json);

    let path = std::path::Path::new("./tests/output/indent_word_online.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_tab_and_break() {
    let mut file = File::open("../fixtures/tab_and_break/tab_and_break.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_tab_and_break", &json);

    let path = std::path::Path::new("./tests/output/tab_and_break.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_table_docx() {
    let mut file = File::open("../fixtures/table_docx/table.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_table_docx", &json);

    let path = std::path::Path::new("./tests/output/table_docx.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_table_merged_libre_office() {
    let mut file = File::open("../fixtures/table_merged_libre_office/table_merged.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_table_merged_libre_office", &json);

    let path = std::path::Path::new("./tests/output/table_merged_libre_office.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_bom() {
    let mut file = File::open("../fixtures/bom/bom.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_bom", &json);

    let path = std::path::Path::new("./tests/output/bom.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_bookmark() {
    let mut file = File::open("../fixtures/bookmark/bookmark.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_bookmark", &json);

    let path = std::path::Path::new("./tests/output/bookmark.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_insert_table() {
    let mut file = File::open("../fixtures/insert_table/insert_table.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_insert_table", &json);

    let path = std::path::Path::new("./tests/output/insert_table.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_textbox() {
    let mut file = File::open("../fixtures/textbox/textbox.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_textbox", &json);

    let path = std::path::Path::new("./tests/output/textbox.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_from_doc() {
    let mut file = File::open("../fixtures/from_doc/from_doc.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_from_doc", &json);

    let path = std::path::Path::new("./tests/output/from_doc.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_lvl_override() {
    let mut file = File::open("../fixtures/lvl_override/override.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_lvl_override", &json);

    let path = std::path::Path::new("./tests/output/lvl_override.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_comment() {
    let mut file = File::open("../fixtures/comment/comment.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_comment", &json);

    let path = std::path::Path::new("./tests/output/comment.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_extended_comment() {
    let mut file = File::open("../fixtures/extended_comments/extended_comments.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_extended_comments", &json);

    let path = std::path::Path::new("./tests/output/extended_comments.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_line_spacing() {
    let mut file = File::open("../fixtures/line_spacing/line_spacing.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("line_spacing", &json);

    let path = std::path::Path::new("./tests/output/line_spacing.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

#[test]
pub fn read_footnotes() {
    let mut file = File::open("../fixtures/footnotes/footnotes.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let json = read_docx(&buf).unwrap().json();

    assert_json_snapshot!("read_footnotes", &json);

    let path = std::path::Path::new("./tests/output/footnotes.json");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}
