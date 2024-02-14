use docx_rs::{Docx, Header, Paragraph, Pic, Run};
use std::{error::Error, io::Cursor};

fn main() -> Result<(), Box<dyn Error>> {
    let cat = Pic::new(include_bytes!("../../images/cat_min.jpg"));
    let header =
        Header::new().add_paragraph(Paragraph::new().add_run(Run::new().add_image(cat.clone())));
    let mut out = Vec::new();
    let docx = Docx::new()
        .header(header)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_image(cat)));
    docx.build().pack(Cursor::new(&mut out))?;
    std::fs::write("/tmp/out.docx", &out)?;
    Ok(())
}
