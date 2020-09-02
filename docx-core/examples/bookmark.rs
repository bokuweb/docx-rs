use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/bookmark.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_bookmark_start(1, "hello")
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .add_bookmark_end(1)
        .build()
        .pack(file)?;
    Ok(())
}
