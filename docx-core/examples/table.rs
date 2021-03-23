use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./table.docx");
    let file = std::fs::File::create(&path).unwrap();

    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .grid_span(2)
                .shading(Shading::new().fill("FF0000")),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
                .vertical_align(VAlignType::Center)
                .vertical_merge(VMergeType::Restart)
                .text_direction(TextDirectionType::TbRlV),
        ]),
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .vertical_merge(VMergeType::Restart),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .vertical_merge(VMergeType::Continue),
        ]),
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .vertical_merge(VMergeType::Continue),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .vertical_merge(VMergeType::Continue),
        ]),
    ])
    .set_grid(vec![2000, 2000, 2000])
    .layout(TableLayoutType::Fixed)
    .indent(1000);
    Docx::new().add_table(table).build().pack(file)?;
    Ok(())
}
