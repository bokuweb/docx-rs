use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./table_border.docx");
    let file = std::fs::File::create(&path).unwrap();

    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .grid_span(2)
                .clear_border(TableCellBorderPosition::Left)
                .clear_border(TableCellBorderPosition::Bottom)
                .clear_border(TableCellBorderPosition::Right),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
                .vertical_align(VAlignType::Center)
                .vertical_merge(VMergeType::Restart),
        ]),
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .vertical_merge(VMergeType::Restart)
                .clear_all_border(),
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
    .indent(1000);
    Docx::new().add_table(table).build().pack(file)?;
    Ok(())
}
