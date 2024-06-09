use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/examples/header_with_page_num.docx");
    let file = std::fs::File::create(path).unwrap();
    let header = Header::new().add_paragraph(
        Paragraph::new()
            .wrap("none")
            .v_anchor("text")
            .h_anchor("margin")
            .x_align("right")
            .add_run(Run::new().add_text("Hello"))
            .add_page_num(PageNum::new()),
    );
    Docx::new()
        .header(header)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("World")))
        .build()
        .pack(file)?;
    Ok(())
}
