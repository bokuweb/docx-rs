pub fn create_special_indent(
    special_indent_kind: Option<docx_rs::SpecialIndentKind>,
    special_indent_size: Option<i32>,
) -> Option<docx_rs::SpecialIndentType> {
    if let Some(kind) = special_indent_kind {
        let size = special_indent_size.unwrap_or(0);
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

// Applies character-unit indents (hundredths of a character) on top of the
// indent already set on the paragraph property. `None` leaves a value unchanged.
pub fn apply_indent_chars(
    mut p: docx_rs::ParagraphProperty,
    start_chars: Option<i32>,
    end_chars: Option<i32>,
    hanging_chars: Option<i32>,
    first_line_chars: Option<i32>,
) -> docx_rs::ParagraphProperty {
    if let Some(chars) = start_chars {
        p = p.start_chars(chars);
    }
    if let Some(chars) = end_chars {
        p = p.end_chars(chars);
    }
    if let Some(chars) = hanging_chars {
        p = p.hanging_chars(chars);
    }
    if let Some(chars) = first_line_chars {
        p = p.first_line_chars(chars);
    }
    p
}
