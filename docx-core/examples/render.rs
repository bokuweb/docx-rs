use std::collections::HashMap;

fn main() {
    let path = r"C:\Users\33028\Desktop\test_render.docx";
    let mut docx = docx_rs::Docx::read_file(path).unwrap();
    let dictionary = HashMap::from([
        ("test1".to_string(), "岳卓".to_string()),
        ("test2".to_string(), "test2_value".to_string()),
        ("test3".to_string(), "test3_value".to_string()),
    ]);
    docx.render(&dictionary);
    let file = std::fs::File::create(r"C:\Users\33028\Desktop\test_render1.docx").unwrap();
    docx.build().pack(file).unwrap();
}
