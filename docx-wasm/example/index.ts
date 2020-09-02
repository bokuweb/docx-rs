import { saveAs } from "file-saver";

import("../js").then((w) => {
  const buf = new w.Docx()
    .addBookmarkStart(0, "Hello")
    .addParagraph(
      new w.Paragraph()
        .addRun(new w.Run().addText("Hello "))
        .addRun(
          new w.Run()
            .addText("World!!")
            .bold()
            .highlight("yellow")
            .color("FF0000")
            .size(40)
        )
        .addCommentStart(
          new w.Comment(1)
            .author("bokuweb")
            .date(new Date().toISOString())
            .paragraph(new w.Paragraph().addRun(new w.Run().addText("hello")))
        )
        .addCommentEnd(new w.CommentEnd(1))
        .addCommentStart(
          new w.Comment(2)
            .author("bokuweb")
            .date(new Date().toISOString())
            .parentCommentId(1)
            .paragraph(new w.Paragraph().addRun(new w.Run().addText("world")))
        )
        .addCommentEnd(new w.CommentEnd(2))
    )
    .addBookmarkEnd(0)
    .docId("12345678-1234-1234-1234-1234567890AB")
    .build();
  saveAs(new Blob([buf]), "hello.docx");
});
