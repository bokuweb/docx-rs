use docx_rs::*;

pub const DUMMY: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/indent.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(DUMMY)).indent(
            Some(840),
            None,
            None,
            None,
        ))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(DUMMY)).indent(
            Some(840),
            Some(SpecialIndentType::FirstLine(720)),
            None,
            None,
        ))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(DUMMY)).indent(
            Some(1560),
            Some(SpecialIndentType::Hanging(720)),
            None,
            None,
        ))
        .build()
        .pack(file)?;
    Ok(())
}
