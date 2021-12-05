use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/dirty_toc.docx");
    let file = std::fs::File::create(&path).unwrap();
    let p = Paragraph::new()
        .add_run(Run::new().add_text("Hello"))
        .style("Heading1")
        .page_break_before(true);
    let style = Style::new("Heading1", StyleType::Paragraph).name("Heading 1");
    Docx::new()
        .add_style(style)
        .add_table_of_contents(TableOfContents::new().heading_styles_range(1, 3))
        .add_paragraph(p)
        .build()
        .pack(file)?;
    Ok(())
}
