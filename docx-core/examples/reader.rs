use docx_rs::*;
use std::fs::*;
use std::io::Read;
use std::path::*;

pub fn main() {
    let mut file = File::open("./fixtures/hello_world/hello_world.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    read_docx(&buf);
}
