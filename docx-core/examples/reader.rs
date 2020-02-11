use docx_rs::*;
use std::fs::*;
use std::io::Read;

pub fn main() {
    let mut file = File::open("./fixtures/paragraph/paragraph.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    read_docx(&buf).unwrap();
}
