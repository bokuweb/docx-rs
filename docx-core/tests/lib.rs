extern crate docx_core;

use docx_core::*;

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
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new().add_text(DUMMY))
        .indent(840, None),
    )
    .add_paragraph(Paragraph::new())
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new().add_text(DUMMY))
        .indent(840, Some(SpecialIndentType::FirstLine(720))),
    )
    .add_paragraph(Paragraph::new())
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new().add_text(DUMMY))
        .indent(1560, Some(SpecialIndentType::Hanging(720))),
    )
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
pub fn custom_attr_paragraph() -> Result<(), DocxError> {
  let path = std::path::Path::new("./tests/output/custom_attr_paragraph.docx");
  let file = std::fs::File::create(&path).unwrap();
  Docx::new()
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new().add_text("Hello"))
        .add_attr("w:customId", "1234-5678"),
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
        .add_insert(Insert::new().run(Run::new().add_text("Hello")))
        .add_delete(Delete::new().run(Run::new().add_delete_text("World"))),
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
          Comment::new("1")
            .author("bokuweb")
            .date("2019-01-01T00:00:00Z")
            .paragraph(Paragraph::new().add_run(Run::new().add_text("Hello"))),
        )
        .add_run(Run::new().add_text("Hello").highlight("cyan"))
        .add_run(Run::new().add_text(" World!").highlight("yellow"))
        .add_comment_end("1"),
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
          Comment::new("1")
            .author("bokuweb")
            .date("2019-01-01T00:00:00Z")
            .paragraph(Paragraph::new().add_run(Run::new().add_text("Hello"))),
        )
        .add_run(Run::new().add_text("Hello"))
        .add_comment_end("1"),
    ),
    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("World"))),
  ])]);
  Docx::new()
    .add_paragraph(
      Paragraph::new()
        .add_comment_start(
          Comment::new("ABCD-1234")
            .author("bokuweb")
            .date("2019-01-01T00:00:00Z")
            .paragraph(Paragraph::new().add_run(Run::new().add_text("Comment!!"))),
        )
        .add_run(Run::new().add_text("Hello").highlight("cyan"))
        .add_comment_end("ABCD-1234"),
    )
    .add_table(table)
    .build()
    .pack(file)?;
  Ok(())
}
