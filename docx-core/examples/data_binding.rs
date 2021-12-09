use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/data_binding.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_structured_data_tag(
                    StructuredDataTag::new().data_binding(DataBinding::new().xpath("/root/item1")),
                )
                .add_structured_data_tag(
                    StructuredDataTag::new().data_binding(DataBinding::new().xpath("/root/item2")),
                ),
        )
        .add_custom_item(
            "06AC5857-5C65-A94A-BCEC-37356A209BC3",
            "<root><item1>Hello</item1><item2> World!</item2></root>",
        )
        .build()
        .pack(file)?;
    Ok(())
}
