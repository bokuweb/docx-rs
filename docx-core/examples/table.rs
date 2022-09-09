use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./table.docx");
    let file = std::fs::File::create(&path).unwrap();

    let table = Table::new(vec![TableRow::new(vec![
        TableCell::new().add_paragraph(Paragraph::new())
    ])]);
    Docx::new().add_table(table).build().pack(file)?;
    Ok(())
}
