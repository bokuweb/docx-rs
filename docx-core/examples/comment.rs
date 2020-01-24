use docx::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./comment.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_comment_start(
                    Comment::new(1)
                        .author("bokuweb")
                        .date("2019-01-01T00:00:00Z")
                        .paragraph(Paragraph::new().add_run(Run::new().add_text("Hello"))),
                )
                .add_comment_end(1),
        )
        .build()
        .pack(file)?;
    Ok(())
}
