extern crate docx_rs;

mod reader;

use docx_rs::*;

pub const DUMMY: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

#[test]
pub fn hello() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/hello.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn indent() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/indent.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(DUMMY)).indent(
            Some(840),
            None,
            None,
            None,
        ))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(DUMMY)).indent(
            Some(840),
            Some(SpecialIndentType::FirstLine(720)),
            None,
            None,
        ))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(DUMMY)).indent(
            Some(1560),
            Some(SpecialIndentType::Hanging(720)),
            None,
            None,
        ))
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn size() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/size.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello").size(60)))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(" Wor").size(50))
                .add_run(Run::new().add_text("ld")),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn alignment() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/alignment.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(" World"))
                .align(AlignmentType::Right),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn table() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/table.docx");
    let file = std::fs::File::create(&path).unwrap();

    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("World"))),
        ]),
        TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Foo"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bar"))),
        ]),
    ]);
    Docx::new().add_table(table).build().pack(file)?;
    Ok(())
}

#[test]
pub fn table_with_grid() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/table_with_grid.docx");
    let file = std::fs::File::create(&path).unwrap();

    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("World"))),
        ]),
        TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Foo"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bar"))),
        ]),
    ])
    .set_grid(vec![3000, 3000]);
    Docx::new().add_table(table).build().pack(file)?;
    Ok(())
}

#[test]
pub fn table_merged() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/table_merged.docx");
    let file = std::fs::File::create(&path).unwrap();

    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .grid_span(2),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
                .vertical_merge(VMergeType::Restart),
        ]),
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .vertical_merge(VMergeType::Restart),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .vertical_merge(VMergeType::Continue),
        ]),
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .vertical_merge(VMergeType::Continue),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new()
                .add_paragraph(Paragraph::new())
                .vertical_merge(VMergeType::Continue),
        ]),
    ])
    .set_grid(vec![2000, 2000, 2000]);
    Docx::new().add_table(table).build().pack(file)?;
    Ok(())
}

#[test]
pub fn decoration() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/decoration.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .add_run(Run::new().add_text(" World").bold()),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .add_run(Run::new().add_text(" World").highlight("yellow")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .add_run(Run::new().add_text(" World").italic()),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .add_run(Run::new().add_text(" World").color("FF0000")),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn tab_and_break() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/tab_and_break.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text("Hello")
                    .add_tab()
                    .add_text("World")
                    .add_break(BreakType::Page)
                    .add_text("Foo"),
            ),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn history() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/history.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_insert(
                    Insert::new(Run::new().add_text("Hello"))
                        .author("bokuweb")
                        .date("2019-01-01T00:00:00Z"),
                )
                .add_delete(Delete::new().add_run(Run::new().add_delete_text("World"))),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn underline() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/underline.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello").underline("single")))
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn highlight() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/highlight.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello").highlight("cyan"))
                .add_run(Run::new().add_text(" World!").highlight("yellow")),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn comments() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/comments.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_comment_start(
                    Comment::new(1)
                        .author("bokuweb")
                        .date("2019-01-01T00:00:00Z")
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello"))),
                )
                .add_run(Run::new().add_text("Hello").highlight("cyan"))
                .add_run(Run::new().add_text(" World!").highlight("yellow"))
                .add_comment_end(1),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn comments_to_table() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/comments_table.docx");
    let file = std::fs::File::create(&path).unwrap();
    let table = Table::new(vec![TableRow::new(vec![
        TableCell::new().add_paragraph(
            Paragraph::new()
                .add_comment_start(
                    Comment::new(1)
                        .author("bokuweb")
                        .date("2019-01-01T00:00:00Z")
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello"))),
                )
                .add_run(Run::new().add_text("Hello"))
                .add_comment_end(1),
        ),
        TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("World"))),
    ])]);
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_comment_start(
                    Comment::new(1)
                        .author("bokuweb")
                        .date("2019-01-01T00:00:00Z")
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Comment!!"))),
                )
                .add_run(Run::new().add_text("Hello").highlight("cyan"))
                .add_comment_end(1),
        )
        .add_table(table)
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn default_numbering() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/default_numbering.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .numbering(NumberingId::new(1), IndentLevel::new(0)),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("World!"))
                .numbering(NumberingId::new(1), IndentLevel::new(1)),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Foooo!"))
                .numbering(NumberingId::new(1), IndentLevel::new(2)),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Bar!"))
                .numbering(NumberingId::new(1), IndentLevel::new(3)),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn user_numbering() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/user_numbering.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .numbering(NumberingId::new(2), IndentLevel::new(0)),
        )
        .add_abstract_numbering(
            AbstractNumbering::new(2).add_level(
                Level::new(
                    0,
                    Start::new(1),
                    NumberFormat::new("decimal"),
                    LevelText::new("Section %1."),
                    LevelJc::new("left"),
                )
                .indent(
                    Some(1620),
                    Some(SpecialIndentType::Hanging(320)),
                    None,
                    None,
                ),
            ),
        )
        .add_numbering(Numbering::new(2, 2))
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn escape() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/escape.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("&&&>>><<"))
                .numbering(NumberingId::new(2), IndentLevel::new(0)),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn vanish() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/vanish.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .add_run(Run::new().add_text("Hidden").vanish())
                .add_run(Run::new().add_text(" World!!")),
        )
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn date() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/date.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .created_at("2019-01-01T00:00:00Z")
        .updated_at("2019-01-02T10:00:00Z")
        .build()
        .pack(file)?;
    Ok(())
}

#[test]
pub fn line_spacing() -> Result<(), DocxError> {
    let path = std::path::Path::new("./tests/output/line_spacing.docx");
    let file = std::fs::File::create(&path).unwrap();

    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(DUMMY))
                .line_spacing(
                    LineSpacing::new()
                        .before(300)
                        .line(300)
                        .line_rule(LineSpacingType::Auto),
                ),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(DUMMY))
                .line_spacing(
                    LineSpacing::new()
                        .line(300)
                        .line_rule(LineSpacingType::AtLeast),
                ),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(DUMMY).character_spacing(100))
                .line_spacing(
                    LineSpacing::new()
                        .after(300)
                        .line(300)
                        .line_rule(LineSpacingType::Exact),
                ),
        )
        .build()
        .pack(file)?;
    Ok(())
}
