use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/first_header.docx");
    let file = std::fs::File::create(&path).unwrap();
    let header =
        Header::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")));
    let first_header =
        Header::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("First")));
    Docx::new()
        .header(header)
        .first_header(first_header)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("World")))
        .build()
        .pack(file)?;
    Ok(())
}
