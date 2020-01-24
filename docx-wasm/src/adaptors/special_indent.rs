use docx;

pub fn create_special_indent(
    special_indent_kind: Option<docx::SpecialIndentKind>,
    special_indent_size: Option<usize>,
) -> Option<docx::SpecialIndentType> {
    if let Some(kind) = special_indent_kind {
        let size = special_indent_size.unwrap_or_else(|| 0);
        match kind {
            docx::SpecialIndentKind::FirstLine => Some(docx::SpecialIndentType::FirstLine(size)),
            docx::SpecialIndentKind::Hanging => Some(docx::SpecialIndentType::Hanging(size)),
        }
    } else {
        None
    }
}
