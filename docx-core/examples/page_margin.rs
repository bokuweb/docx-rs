use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/page_margin.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .page_margin(
            PageMargin::new()
                .top(3200)
                .footer(3200)
                .left(3200)
                .right(3200),
        )
        .build()
        .pack(file)?;
    Ok(())
}
