use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/sdt.docx");
    let file = std::fs::File::create(&path).unwrap();
    let p = Paragraph::new().add_run(
        Run::new()
            .add_text("Hello")
            .fonts(RunFonts::new().ascii("Arial")),
    );
    Docx::new()
        .add_structured_data_tag(StructuredDataTag::new().add_paragraph(p))
        .build()
        .pack(file)?;
    Ok(())
}
