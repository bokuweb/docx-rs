import { saveAs } from "file-saver";

import("docx-wasm").then(w => {
  const buf = new w.Docx()
    .addParagraph(
      new w.Paragraph().addRun(new w.Run().addText("Hello world!!"))
    )
    .build();
  saveAs(new Blob([buf]), "hello.docx");
});
