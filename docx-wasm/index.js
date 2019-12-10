// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!
const rust = import("./pkg");
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`

rust
  .then(m => {
    const p = m.createParagraph().add_run(
      m
        .createRun()
        .add_text("Hello World!!")
        .bold()
    );
    const t = m
      .createTable()
      .add_row(
        m.createTableRow().add_cell(m.createTableCell().add_paragraph(p))
      );
    let docx = m.createDocx().add_table(t);
    saveAs(new Blob([docx.build()]), "example.docx");
    docx.free();
  })
  .catch(console.error);
