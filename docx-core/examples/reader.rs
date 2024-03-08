use docx_rs::*;

use std::fs::File;
use std::io::{Read, Write};

pub fn main() {
    let mut file = File::open("./hello.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();

    let mut file = File::create("./hello.json").unwrap();
    let res = read_docx(&buf).unwrap().json();
    file.write_all(res.as_bytes()).unwrap();
    file.flush().unwrap();
}
