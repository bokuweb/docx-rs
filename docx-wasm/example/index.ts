import { saveAs } from "file-saver";

import("../js").then(w => {
  const buf = new w.Docx()
    .addParagraph(
      new w.Paragraph().addRun(new w.Run().addText("Hello ")).addRun(
        new w.Run()
          .addText("World!!")
          .bold()
          .highlight("yellow")
          .color("FF0000")
      )
    )
    .build();
  saveAs(new Blob([buf]), "hello.docx");
});
