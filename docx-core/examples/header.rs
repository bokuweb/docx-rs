use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/header.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_header_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("World")))
        .build()
        .pack(file)?;
    Ok(())
}
