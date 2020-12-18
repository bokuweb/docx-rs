const w = require("../dist/node");
const { readFileSync, writeFileSync } = require("fs");
const Zip = require("adm-zip");

describe("reader", () => {
  test("should read lvlOverride docx", () => {
    const buf = readFileSync("../fixtures/lvl_override/override.docx");
    const json = w.readDocx(buf);
    expect(json).toMatchSnapshot();
  });

  test("should read gridAfter docx", () => {
    const buf = readFileSync("../fixtures/grid_after/grid_after.docx");
    const json = w.readDocx(buf);
    expect(json).toMatchSnapshot();
  });

  test("should read table style docx", () => {
    const buf = readFileSync("../fixtures/table_style/table_style.docx");
    const json = w.readDocx(buf);
    expect(json).toMatchSnapshot();
  });

  test("should read extended comments docx", () => {
    const buf = readFileSync(
      "../fixtures/extended_comments/extended_comments.docx"
    );
    const json = w.readDocx(buf);
    expect(json).toMatchSnapshot();
  });
});

describe("writer", () => {
  test("should write hello", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!"));
    const buf = new w.Docx().addParagraph(p).build();
    const z = new Zip(Buffer.from(buf));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write lvlOverride with level", () => {
    const p = new w.Paragraph()
      .addRun(new w.Run().addText("Hello world!!"))
      .numbering(0, 0);
    const num = new w.Numbering(0, 0);
    num.addOverride(
      new w.LevelOverride(0).overrideLevel(
        new w.Level(0, 3, "decimal", "%1", "left")
      )
    );
    const buf = new w.Docx()
      .addParagraph(p)
      .addAbstractNumbering(new w.AbstractNumbering(0))
      .addNumbering(num)
      .build();

    const z = new Zip(Buffer.from(buf));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write page size", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!"));
    const buf = new w.Docx().addParagraph(p).pageSize(400, 800).build();
    const z = new Zip(Buffer.from(buf));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write page margin", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!"));
    const buf = new w.Docx()
      .addParagraph(p)
      .pageMargin({ top: 1000, left: 2000 })
      .build();
    const z = new Zip(Buffer.from(buf));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write default font", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!!!"));
    const fonts = new w.RunFonts()
      .eastAsia("Arial")
      .ascii("Arial")
      .hiAnsi("Arial");
    const buf = new w.Docx()
      .addParagraph(p)
      .defaultSize(40)
      .defaultFonts(fonts)
      .build();
    writeFileSync("../output/default_font.docx", buf);
    const z = new Zip(Buffer.from(buf));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write doc vars", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!!!"));
    const buf = new w.Docx().addParagraph(p).addDocVar("foo", "bar").build();
    writeFileSync("../output/doc_vars.docx", buf);
    const z = new Zip(Buffer.from(buf));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });
});
