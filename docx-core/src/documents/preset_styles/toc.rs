use crate::documents::*;
use crate::types::*;

pub fn toc(level: i32) -> Style {
    Style::new(format!("ToC{}", level), StyleType::Paragraph)
        .name(format!("toc {}", level))
        .align(AlignmentType::Both)
        .snap_to_grid(false)
        .indent(Some((level - 1) * 200), None, None, Some((level - 1) * 100))
}
