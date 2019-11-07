mod documents;
mod types;
mod xml_builder;
mod zipper;

pub use documents::*;
use types::*;
use zipper::*;

pub fn simple() {
    let xml = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new("Hello")))
        .build();
    zip("./test.docx", xml);
}
