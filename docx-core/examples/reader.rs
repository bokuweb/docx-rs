use docx_rs::*;
use std::fs::*;
use std::io::Read;

pub fn main() {
    let mut file = File::open("./insert.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    dbg!(read_docx(&buf).unwrap().json());
}
