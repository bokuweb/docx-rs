use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./history.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_insert(
                    Insert::new(Run::new().add_text("Hello"))
                        .author("bokuweb")
                        .date("2019-01-01T00:00:00Z"),
                )
                .add_delete(Delete::new().add_run(Run::new().add_delete_text("World"))),
        )
        .build()
        .pack(file)?;
    Ok(())
}
