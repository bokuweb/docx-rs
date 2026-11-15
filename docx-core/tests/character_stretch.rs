use docx_rs::*;
use std::io::Cursor;

#[test]
fn character_stretch_survives_package_read_and_rebuild() {
    let scales = [Some(75), Some(100), Some(125), Some(160), None];
    let mut paragraph = Paragraph::new();
    for scale in scales {
        let mut run = Run::new().add_text("Scale sample").bold().size(22);
        if let Some(scale) = scale {
            run = run.stretch(scale);
        }
        paragraph = paragraph.add_run(run);
    }
    let mut document = Docx::new().add_paragraph(paragraph);

    for _ in 0..2 {
        let mut package = Cursor::new(Vec::new());
        document.build().pack(&mut package).unwrap();
        document = read_docx(package.get_ref()).unwrap();
        let DocumentChild::Paragraph(paragraph) = &document.document.children[0] else {
            panic!("the paragraph must survive the package round trip")
        };
        assert_eq!(paragraph.children.len(), scales.len());
        for (child, scale) in paragraph.children.iter().zip(scales) {
            let ParagraphChild::Run(run) = child else {
                panic!("each source run must survive the package round trip")
            };
            assert_eq!(run.run_property.stretch, scale.map(Stretch::new));
            assert_eq!(run.run_property.bold, Some(Bold::new()));
            assert_eq!(run.run_property.sz, Some(Sz::new(22)));
            assert_eq!(
                run.children,
                vec![RunChild::Text(Text::new("Scale sample"))]
            );
        }
    }
}
