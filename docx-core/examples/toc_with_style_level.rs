use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/examples/toc_with_style_level.docx");
    let file = std::fs::File::create(&path).unwrap();

    let style1 = Style::new("Heading1", StyleType::Paragraph).name("Heading 1");
    let style2 = Style::new("StyleLevel1", StyleType::Paragraph)
        .name("Style Level1")
        .based_on("Heading1");
    let style3 = Style::new("StyleLevel4", StyleType::Paragraph)
        .name("Style Level4")
        .based_on("Heading4");
    let style4 = Style::new("Heading4", StyleType::Paragraph).name("Heading 4");

    let p1 = Paragraph::new()
        .add_run(Run::new().add_text("Hello"))
        .style("Heading1")
        .page_break_before(true);

    let p2 = Paragraph::new()
        .add_run(Run::new().add_text("Foo"))
        .style("StyleLevel1")
        .page_break_before(true);

    let p3 = Paragraph::new()
        .add_run(Run::new().add_text("Bar"))
        .style("StyleLevel4")
        .page_break_before(true);
    Docx::new()
        .add_style(style1)
        .add_style(style2)
        .add_style(style3)
        .add_style(style4)
        .add_table_of_contents(
            TableOfContents::new()
                .heading_styles_range(1, 3)
                .add_style_with_level(StyleWithLevel::new("StyleLevel1", 1))
                .add_style_with_level(StyleWithLevel::new("StyleLevel4", 4))
                .alias("Table of contents")
                .auto(),
        )
        .add_paragraph(p1)
        .add_paragraph(p2)
        .add_paragraph(p3)
        .build()
        .pack(file)?;
    Ok(())
}
