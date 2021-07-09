use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/web_ext.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .taskpanes()
        .web_extension(
            WebExtension::new(
                "7f33b723-fb58-4524-8733-dbedc4b7c095",
                "abc",
                "1.0.0.0",
                "developer",
                "Registry",
            )
            .property("hello", "\"world\""),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .build()
        .pack(file)?;
    Ok(())
}
