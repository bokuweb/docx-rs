mod documents;
mod errors;
mod types;
mod xml_builder;
mod zipper;

pub use documents::*;
pub use errors::*;
pub use types::*;

pub fn simple() {
    let path = std::path::Path::new("./test.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(" World")))
        .build()
        .pack(file);
}
