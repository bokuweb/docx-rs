<p align="center"><img src ="https://github.com/bokuweb/docx-rs/blob/master/logo.png?raw=true" /></p>

<p align="center">
    A .docx file reader/writer  with Rust/WebAssembly.
</p>

---

[![GitHub Actions Status](https://github.com/bokuweb/docx-rs/workflows/Continuous%20Integration/badge.svg)](https://github.com/bokuweb/docx-rs/actions)

## Installation

### Rust

```
[dependencies]
docx-rs = "0.2.0"
```

### Browser/Node.js

```
yarn add docx-wasm
```

## Example

### Rust

```rust
use docx_rs::*;

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

### Browser

```javascript
import { saveAs } from "file-saver";

// // Note that a dynamic `import` statement here is required due to webpack/webpack#6615,
import("docx-wasm").then(w => {
  const buf = new w.Docx()
    .addParagraph(
      new w.Paragraph().addRun(new w.Run().addText("Hello world!!"))
    )
    .build();
  saveAs(new Blob([buf]), "hello.docx");
});
```

### Node.js

```javascript
const w = require("docx-wasm");
const { writeFileSync } = require("fs");

const buf = new w.Docx()
  .addParagraph(new w.Paragraph().addRun(new w.Run().addText("Hello world!!")))
  .build();

writeFileSync("hello.docx", buf);
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
- [ ] Image
- [x] Style
- [x] Table
- [x] HIstory
- [ ] Table of contents
- [ ] Section
