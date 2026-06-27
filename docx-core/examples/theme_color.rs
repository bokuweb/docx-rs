use docx_rs::*;

// Demonstrates theme-aware colors. The runs reference theme colors
// (accent1 / hyperlink) so Word re-resolves them when the document theme
// changes, while the w:val hex stays as a fallback for theme-unaware renderers.
pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/theme_color.docx");
    let file = std::fs::File::create(path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text("Accent1 heading")
                    .color("2E74B5")
                    .theme_color(ThemeColor::Accent1)
                    .theme_shade("BF"),
            ),
        )
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text("Hyperlink-colored text")
                    .color("0563C1")
                    .theme_color(ThemeColor::Hyperlink),
            ),
        )
        .build()
        .pack(file)?;
    Ok(())
}
