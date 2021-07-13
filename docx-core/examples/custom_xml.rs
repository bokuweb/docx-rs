use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/custom_xml.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .add_custom_item("06AC5857-5C65-A94A-BCEC-37356A209BC3", "<root xmlns=\"https://exampple.com\"><item name=\"Cheap Item\" price=\"$193.95\"/><item name=\"Expensive Item\" price=\"$931.88\"/></root>")
        .build()
        .pack(file)?;
    Ok(())
}
