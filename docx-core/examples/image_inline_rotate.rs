use std::fs::*;
use std::io::Read;

use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/examples/image_inline_rotate.docx");
    let file = File::create(&path).unwrap();
    let mut img = File::open("./images/cat_min.jpg").unwrap();
    let mut buf = Vec::new();
    let _ = img.read_to_end(&mut buf).unwrap();

    // rotate 180deg.
    let pic = Pic::new(&buf).size(320 * 9525, 240 * 9525).rotate(180);
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("🐱").add_image(pic)))
        .build()
        .pack(file)?;
    Ok(())
}
