pub fn create_special_indent(
    special_indent_kind: Option<docx_rs::SpecialIndentKind>,
    special_indent_size: Option<i32>,
) -> Option<docx_rs::SpecialIndentType> {
    if let Some(kind) = special_indent_kind {
        let size = special_indent_size.unwrap_or_else(|| 0);
        match kind {
            docx_rs::SpecialIndentKind::FirstLine => {
                Some(docx_rs::SpecialIndentType::FirstLine(size))
            }
            docx_rs::SpecialIndentKind::Hanging => Some(docx_rs::SpecialIndentType::Hanging(size)),
        }
    } else {
        None
    }
}
