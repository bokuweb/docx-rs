use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/page_margin.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .page_margin(
            PageMargin::new()
                .top(3000)
                .footer(3000)
                .left(3000)
                .right(3000),
        )
        .build()
        .pack(file)?;
    Ok(())
}
