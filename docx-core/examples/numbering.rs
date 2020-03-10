use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./numbering.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .numbering(NumberingId::new(2), IndentLevel::new(0)),
        )
        .add_abstract_numbering(
            AbstractNumbering::new(2).add_level(
                Level::new(
                    0,
                    Start::new(1),
                    NumberFormat::new("decimal"),
                    LevelText::new("Section %1."),
                    LevelJc::new("left"),
                )
                .indent(
                    Some(1620),
                    Some(SpecialIndentType::Hanging(320)),
                    None,
                    None,
                ),
            ),
        )
        .add_numbering(Numbering::new(2, 2))
        .build()
        .pack(file)?;
    Ok(())
}
