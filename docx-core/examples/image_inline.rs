use std::fs::*;
use std::io::Read;

use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/image.docx");
    let file = File::create(&path).unwrap();
    let mut img = File::open("./images/cat_min.jpg").unwrap();
    let mut buf = Vec::new();
    let _ = img.read_to_end(&mut buf).unwrap();

    let pic = Pic::new(buf).size(320, 240);
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("🐱").add_image(pic)))
        .build()
        .pack(file)?;
    Ok(())
}
