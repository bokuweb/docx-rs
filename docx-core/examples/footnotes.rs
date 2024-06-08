use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./footnotes.docx");
    let file = std::fs::File::create(path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .add_run(
                    Run::new().add_footnote_reference(
                        Footnote::new(1)
                            .add_content(Paragraph::new().add_run(Run::new().add_text("World"))),
                    ),
                ),
        )
        .build()
        .pack(file)?;
    Ok(())
}
