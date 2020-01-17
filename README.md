<p align="center"><img src ="https://github.com/bokuweb/docx-rs/blob/master/logo.png?raw=true" /></p>

<p align="center">
    A .docx file generater with Rust/WebAssembly.
</p>

---

[![GitHub Actions Status](https://github.com/bokuweb/docx-rs/workflows/Continuous%20Integration/badge.svg)](https://github.com/bokuweb/docx-rs/actions)

## Example

``` rust
use docx_core::*;

pub fn hello() -> Result<(), DocxError> {
  let path = std::path::Path::new("./tests/output/hello.docx");
  let file = std::fs::File::create(&path).unwrap();
  Docx::new()
    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
    .build()
    .pack(file)?;
  Ok(())
}
```

### More examples

* [Indent](https://github.com/bokuweb/docx-rs/blob/master/docx-core/examples/indent.rs)

## Features

- [x] Paragraph
