use docx_core::*;

pub fn main() {
  let path = std::path::Path::new("./output/alignment.docx");
  let file = std::fs::File::create(&path).unwrap();
  Docx::new()
    .add_paragraph(Paragraph::new().add_run(Run::new("Hello")))
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new(" World"))
        .align(AlignmentType::Right),
    )
    .build()
    .pack(file);
}
