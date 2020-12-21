use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/nested_comment.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_comment_start(
                    Comment::new(1)
                        .author("bokuweb")
                        .date("2019-01-01T00:00:00Z")
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("World"))),
                )
                .add_comment_end(1)
                .add_comment_start(
                    Comment::new(2)
                        .author("bokuweb")
                        .date("2019-01-02T00:00:00Z")
                        .parent_comment_id(1)
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("World"))),
                )
                .add_comment_end(2)
                .add_comment_start(
                    Comment::new(3)
                        .author("bokuweb")
                        .date("2019-01-02T00:00:00Z")
                        .parent_comment_id(1)
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("!!!!!")))
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("!!!!!"))),
                )
                .add_comment_end(3),
        )
        .build()
        .pack(file)?;
    Ok(())
}
