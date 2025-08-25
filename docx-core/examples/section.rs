use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/examples/section.docx");
    let file = std::fs::File::create(path).unwrap();
    let section = Section::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .page_size(PageSize::new().size(10000, 10000))
        .page_orient(PageOrientationType::Landscape)
        .header(
            Header::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Header"))),
        );

    Docx::new()
        .add_section(section)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("World")))
        .header(Header::new().add_paragraph(Paragraph::new()))
        .build()
        .pack(file)?;
    Ok(())
}
