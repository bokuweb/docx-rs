use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/doc_id.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("World")))
        .doc_id("2F0CF1F9-607F-5941-BF59-8A81BE87AAAA")
        .build()
        .pack(file)?;
    Ok(())
}
