use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/toc_with_item.docx");
    let file = std::fs::File::create(&path).unwrap();
    let p1 = Paragraph::new()
        .add_run(Run::new().add_text("Hello"))
        .style("Heading1")
        .page_break_before(true);
    let style1 = Style::new("Heading1", StyleType::Paragraph).name("Heading 1");
    let p2 = Paragraph::new()
        .add_run(Run::new().add_text("World"))
        .style("Heading2")
        .page_break_before(true);
    let style2 = Style::new("Heading2", StyleType::Paragraph).name("Heading 2");

    Docx::new()
        .add_style(style1)
        .add_style(style2)
        .add_table_of_contents(
            TableOfContents::new()
                .alias("Table of contents")
                .heading_styles_range(1, 3)
                .add_item(
                    TableOfContentsItem::new()
                        .text("Hello")
                        .toc_key("_Toc00000000")
                        .level(1)
                        .page_ref("2"),
                )
                .add_item(
                    TableOfContentsItem::new()
                        .text("World")
                        .toc_key("_Toc00000001")
                        .level(2)
                        .page_ref("3"),
                ),
        )
        .add_paragraph(p1)
        .add_paragraph(p2)
        .build()
        .pack(file)?;
    Ok(())
}
