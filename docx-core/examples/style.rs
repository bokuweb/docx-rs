use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/examples/style.docx");
    let file = std::fs::File::create(&path).unwrap();

    let p1 = Paragraph::new()
        .add_run(Run::new().add_text("Hello").style("Run1"))
        .add_run(Run::new().add_text(" World"))
        .style("Heading1")
        .page_break_before(true);

    let style1 = Style::new("Heading1", StyleType::Paragraph)
        .name("Heading 1")
        .align(AlignmentType::Center);

    let style2 = Style::new("Run1", StyleType::Character)
        .name("Run test")
        .bold();

    Docx::new()
        .add_style(style1)
        .add_style(style2)
        .add_paragraph(p1)
        .build()
        .pack(file)?;
    Ok(())
}
