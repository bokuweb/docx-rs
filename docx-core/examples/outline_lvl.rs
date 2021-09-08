use docx_rs::*;

fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./outlineLvl.docx");
    let file = std::fs::File::create(&path).unwrap();

    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Title1"))
                .outline_lvl(1),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Title2"))
                .outline_lvl(1),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Title2-1"))
                .outline_lvl(2),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Title2-2"))
                .outline_lvl(2),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Title3"))
                .outline_lvl(1),
        )
        .build()
        .pack(file)?;

    Ok(())
}
