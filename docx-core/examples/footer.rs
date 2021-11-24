use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/footer.docx");
    let file = std::fs::File::create(&path).unwrap();
    let footer =
        Footer::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")));
    Docx::new()
        .footer(footer)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("World")))
        .build()
        .pack(file)?;
    Ok(())
}
