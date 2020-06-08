use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./font.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text("Hello")
                    .fonts(RunFonts::new().ascii("Arial")),
            ),
        )
        .build()
        .pack(file)?;
    Ok(())
}
