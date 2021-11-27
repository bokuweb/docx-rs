use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/even_header.docx");
    let file = std::fs::File::create(&path).unwrap();
    let header =
        Header::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")));
    let even_header =
        Header::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Even")));
    Docx::new()
        .header(header)
        .even_header(even_header)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("World")))
        .build()
        .pack(file)?;
    Ok(())
}
