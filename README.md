<p align="center"><img src ="https://github.com/bokuweb/docx-rs/blob/master/logo.png?raw=true" /></p>

<p align="center">
    A .docx file generater with Rust/WebAssembly.
</p>

---

[![GitHub Actions Status](https://github.com/bokuweb/docx-rs/workflows/Continuous%20Integration/badge.svg)](https://github.com/bokuweb/docx-rs/actions)

## Example

```rust
use docx_core::*;

pub fn hello() -> Result<(), DocxError> {
  let path = std::path::Path::new("./hello.docx");
  let file = std::fs::File::create(&path).unwrap();
  Docx::new()
    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
    .build()
    .pack(file)?;
  Ok(())
}
```

### More examples

- [Minimum](https://github.com/bokuweb/docx-rs/blob/master/docx-core/examples/hello.rs)
- [Indent](https://github.com/bokuweb/docx-rs/blob/master/docx-core/examples/indent.rs)
- [Alignment](https://github.com/bokuweb/docx-rs/blob/master/docx-core/examples/alignment.rs)
- [Numbering](https://github.com/bokuweb/docx-rs/blob/master/docx-core/examples/numbering.rs)
- [Table](https://github.com/bokuweb/docx-rs/blob/master/docx-core/examples/table.rs)
- [Comment](https://github.com/bokuweb/docx-rs/blob/master/docx-core/examples/comment.rs)
- [History](https://github.com/bokuweb/docx-rs/blob/master/docx-core/examples/history.rs)

## Features

- [x] Paragraph
- [x] Alignment
- [x] Indent
- [x] Numbering
- [x] Run
- [x] Bold
- [x] Size
- [x] Color
- [x] Highlight
- [x] Underline
- [x] vanish
- [x] Italic
- [x] Break
- [ ] Header
- [ ] Footer
- [x] Comment
- [x]
- [ ] Image
- [x] Style
- [x] Table
- [x] HIstory
- [ ] Table of contents
- [ ] Section
