use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/toc_with_tc.docx");
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
    let tc = Paragraph::new()
        .add_run(
            Run::new()
                .add_field_char(FieldCharType::Begin, false)
                .add_instr_text(InstrText::TC(InstrTC::new("tc_test").level(4)))
                .add_field_char(FieldCharType::Separate, false)
                .add_field_char(FieldCharType::End, false),
        )
        .page_break_before(true);

    Docx::new()
        .add_style(style1)
        .add_table_of_contents(
            TableOfContents::new().heading_styles_range(1, 3), // .tc_field_level_range(3, 4),
        )
        .add_paragraph(p1)
        .add_paragraph(p2)
        .add_paragraph(tc)
        .build()
        .pack(file)?;
    Ok(())
}
