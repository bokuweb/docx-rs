use docx_core;

pub fn create_special_indent(
    special_indent_kind: Option<docx_core::SpecialIndentKind>,
    special_indent_size: Option<usize>,
) -> Option<docx_core::SpecialIndentType> {
    if let Some(kind) = special_indent_kind {
        let size = if special_indent_size.is_some() {
            special_indent_size.unwrap()
        } else {
            0
        };
        match kind {
            docx_core::SpecialIndentKind::FirstLine => {
                Some(docx_core::SpecialIndentType::FirstLine(size))
            }
            docx_core::SpecialIndentKind::Hanging => {
                Some(docx_core::SpecialIndentType::Hanging(size))
            }
        }
    } else {
        None
    }
}
