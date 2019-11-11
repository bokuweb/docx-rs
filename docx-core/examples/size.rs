use docx_core::*;

pub fn main() -> Result<(), DocxError> {
  let path = std::path::Path::new("./output/size.docx");
  let file = std::fs::File::create(&path).unwrap();
  Docx::new()
    .add_paragraph(Paragraph::new().add_run(Run::new("Hello")).size(60))
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new(" Wor").size(50))
        .add_run(Run::new("ld")),
    )
    .build()
    .pack(file)?;
  Ok(())
}
