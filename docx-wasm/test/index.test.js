const w = require("../dist/node");
const { readFileSync, writeFileSync } = require("fs");
const Zip = require("adm-zip");

const { encodedCat } = require("./encoded-cat");

describe("reader", () => {
  test("should read lvlOverride docx", () => {
    const buffer = readFileSync("../fixtures/lvl_override/override.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read gridAfter docx", () => {
    const buffer = readFileSync("../fixtures/grid_after/grid_after.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read tr2bl docx", () => {
    const buffer = readFileSync("../fixtures/tr2bl/tr2bl.docx");
    const json = w.readDocx(buffer);
    writeFileSync("../output/js/tr2bl.json", JSON.stringify(json, null, 2));
    expect(json).toMatchSnapshot();
  });

  test("should read custom docx", () => {
    const buffer = readFileSync("../fixtures/custom/custom.docx");
    const json = w.readDocx(buffer);
    writeFileSync("../output/js/custom.json", JSON.stringify(json, null, 2));
    expect(json).toMatchSnapshot();
  });

  test("should read table style docx", () => {
    const buffer = readFileSync("../fixtures/table_style/table_style.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read extended comments docx", () => {
    const buffer = readFileSync(
      "../fixtures/extended_comments/extended_comments.docx"
    );
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read multi paragraph comments docx", () => {
    const buf = readFileSync(
      "../fixtures/multi_paragraph_comment/multi_paragraph_comment.docx"
    );
    const json = w.readDocx(buf);
    expect(json).toMatchSnapshot();
  });

  test("should read div docx", () => {
    const buffer = readFileSync("../fixtures/div/div.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read vertAlign docx", () => {
    const buffer = readFileSync("../fixtures/vert_align/vert_align.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read line spacing docx", () => {
    const buffer = readFileSync("../fixtures/line_spacing/line_spacing.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read outlineLvl docx", () => {
    const buffer = readFileSync("../fixtures/outline_lvl/outline_lvl.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read afterLines docx", () => {
    const buffer = readFileSync("../fixtures/after_lines/after_lines.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read toc1 docx", () => {
    const buffer = readFileSync("../fixtures/toc1/toc1.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read footer docx", () => {
    const buffer = readFileSync("../fixtures/footer/footer.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read strike docx", () => {
    const buffer = readFileSync("../fixtures/strike/strike.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read paragraph property change docx", () => {
    const buffer = readFileSync(
      "../fixtures/paragraph_property_change/paragraph_property_change.docx"
    );
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read table indent docx", () => {
    const buffer = readFileSync("../fixtures/table_indent/table_indent.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read del_in_ins docx", () => {
    const buffer = readFileSync("../fixtures/del_in_ins/del_in_ins.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read font docx", () => {
    const buffer = readFileSync("../fixtures/font/font.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read image inline and anchor docx", () => {
    const buffer = readFileSync(
      "../fixtures/image_inline_and_anchor/image_inline_and_anchor.docx"
    );
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read textbox", () => {
    const buffer = readFileSync("../fixtures/textbox/textbox.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read without num id", () => {
    const buffer = readFileSync("../fixtures/without_numid/without_numid.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read nested table", () => {
    const buffer = readFileSync("../fixtures/nested_table/nested_table.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read hyperlink", () => {
    const buffer = readFileSync("../fixtures/link/link.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read hyperlink instr", () => {
    const buffer = readFileSync("../fixtures/instr_links/instr_links.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read sectionProperty in ppr", () => {
    const buffer = readFileSync("../fixtures/section_property_in_ppr/section_property_in_ppr.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });

  test("should read toc0", () => {
    const buffer = readFileSync("../fixtures/toc0/toc0.docx");
    const json = w.readDocx(buffer);
    expect(json).toMatchSnapshot();
  });
});

describe("writer", () => {
  test("should write hello", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!"));
    const buffer = new w.Docx().addParagraph(p).build();
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write align", () => {
    const p = new w.Paragraph()
      .addRun(new w.Run().addText("Hello world!!"))
      .align("both");
    const buffer = new w.Docx().addParagraph(p).build();
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
    writeFileSync("../output/js/align.docx", buffer);
    
  });

  test("should write strike", () => {
    const p = new w.Paragraph().addRun(
      new w.Run().addText("Hello world!!").strike()
    );
    const buffer = new w.Docx().addParagraph(p).build();
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
    writeFileSync("../output/js/strike.docx", buffer);
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
    const buffer = new w.Docx()
      .addParagraph(p)
      .addAbstractNumbering(new w.AbstractNumbering(0))
      .addNumbering(num)
      .build();

    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write page size", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!"));
    const buffer = new w.Docx().addParagraph(p).pageSize(400, 800).build();
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write nested table", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!"));
    const childTable = new w.Table().addRow(
      new w.TableRow().addCell(new w.TableCell().addParagraph(p))
    );
    const parentTable = new w.Table().addRow(
      new w.TableRow().addCell(new w.TableCell().addTable(childTable))
    );
    const buffer = new w.Docx().addTable(parentTable).build();
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
    writeFileSync("../output/js/nested_table.docx", buffer);
  });

  test("should write tl2br and tr2bl cells", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello!!"));
    const table = new w.Table().addRow(
      new w.TableRow()
        .addCell(
          new w.TableCell()
            .setBorder(new w.TableCellBorder("tl2br"))
            .addParagraph(p)
        )
        .addCell(
          new w.TableCell()
            .setBorder(new w.TableCellBorder("tr2bl"))
            .addParagraph(p)
        )
        .addCell(
          new w.TableCell()
            .setBorder(new w.TableCellBorder("tr2bl"))
            .setBorder(new w.TableCellBorder("tl2br"))
            .addParagraph(p)
        )
    );
    const buffer = new w.Docx().addTable(table).build();
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
    writeFileSync("../output/js/cell_borders.docx", buffer);
  });

  test("should write cell shading", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello!!"));
    const table = new w.Table().addRow(
      new w.TableRow().addCell(
        new w.TableCell().addParagraph(p).shading("clear", "auto", "FF0000")
      )
    );
    const buffer = new w.Docx().addTable(table).build();
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
    writeFileSync("../output/js/cell_shading.docx", buffer);
  });

  test("should write page margin", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!"));
    const buffer = new w.Docx()
      .addParagraph(p)
      .pageMargin({ top: 1000, left: 2000 })
      .build();
    const z = new Zip(Buffer.from(buffer));
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
    const buffer = new w.Docx()
      .addParagraph(p)
      .defaultSize(40)
      .defaultFonts(fonts)
      .build();
    writeFileSync("../output/js/default_font.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write doc vars", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!!!"));
    const buffer = new w.Docx().addParagraph(p).addDocVar("foo", "bar").build();
    writeFileSync("../output/js/doc_vars.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write doc grid", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello world!!!!"));
    const buffer = new w.Docx().addParagraph(p).docGrid("default", 360).build();
    writeFileSync("../output/js/doc_grid.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write table layout", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello!!"));
    const table = new w.Table()
      .addRow(new w.TableRow().addCell(new w.TableCell().addParagraph(p)))
      .layout("fixed");
    const buffer = new w.Docx().addTable(table).build();
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
    writeFileSync("../output/js/table_layout.docx", buffer);
  });

  test("should write text border", () => {
    const p = new w.Paragraph()
      .addRun(new w.Run().addText("Hello "))
      .addRun(new w.Run().addText("World!").textBorder("single", 4, 0, "auto"));
    const buffer = new w.Docx().addParagraph(p).build();
    writeFileSync("../output/js/text_border.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write page orientation", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello "));
    const buffer = new w.Docx()
      .addParagraph(p)
      .pageSize(16838, 11906)
      .pageOrientation("landscape")
      .build();
    writeFileSync("../output/js/page_orientation.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write custom props", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello!!"));
    const buffer = new w.Docx()
      .addParagraph(p)
      .customProperty("hello", '{"world": 0}')
      .build();
    writeFileSync("../output/js/custom.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml|custom.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write webextension", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello!!"));
    const buffer = new w.Docx()
      .addParagraph(p)
      .taskpanes()
      .webextension(
        new w.WebExtension(
          "7f33b723-fb58-4524-8733-dbedc4b7c095",
          "abcd",
          "1.0.0.0",
          "developer",
          "Registry"
        ).property("hello", JSON.stringify({ hello: "world" }))
      )
      .build();
    writeFileSync("../output/js/webextension.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/webextension1.xml|_rels|taskpanes.xml.rel/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write customItem", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello!!"));
    const buffer = new w.Docx()
      .addParagraph(p)
      .addCustomItem(
        "06AC5857-5C65-A94A-BCEC-37356A209BC3",
        '<root xmlns="https://example.com"><item name="Cheap Item" price="$193.95"/><item name="Expensive Item" price="$931.88"/></root>'
      )
      .build();
    writeFileSync("../output/js/custom-item.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/item1.xml|_rels|item1Props/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write line spacing", () => {
    const p = new w.Paragraph()
      .addRun(new w.Run().addText("Hello "))
      .lineSpacing(
        new w.LineSpacing().before(100).after(0).line(100).afterLines(400)
      );
    const buffer = new w.Docx().addParagraph(p).build();
    writeFileSync("../output/js/line_spacing.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write footer for default section", () => {
    const p1 = new w.Paragraph().addRun(new w.Run().addText("Hello Footer"));
    const p2 = new w.Paragraph().addRun(new w.Run().addText("World "));
    const footer = new w.Footer().addParagraph(p1);
    const buffer = new w.Docx().footer(footer).addParagraph(p2).build();
    writeFileSync("../output/js/footer.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|footer1.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write header for default section", () => {
    const p1 = new w.Paragraph().addRun(new w.Run().addText("Hello Header"));
    const p2 = new w.Paragraph().addRun(new w.Run().addText("World "));
    const header = new w.Header().addParagraph(p1);
    const buffer = new w.Docx().header(header).addParagraph(p2).build();
    writeFileSync("../output/js/header.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|footer1.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write firstHeader with table for default section", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello Header!!"));
    const table = new w.Table().addRow(
      new w.TableRow().addCell(new w.TableCell().addParagraph(p))
    );
    const header = new w.Header().addTable(table);
    const buffer = new w.Docx().firstHeader(header).build();
    writeFileSync("../output/js/first_header_with_table.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|header[1-9].xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write evenFooter with table for default section", () => {
    const p = new w.Paragraph().addRun(new w.Run().addText("Hello Footer!!"));
    const table = new w.Table().addRow(
      new w.TableRow().addCell(new w.TableCell().addParagraph(p))
    );
    const footer = new w.Footer().addTable(table);
    const buffer = new w.Docx().evenFooter(footer).build();
    writeFileSync("../output/js/even_footer_with_table.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|footer[1-9].xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write anchor hyperlink", () => {
    const p1 = new w.Paragraph().addHyperlink(
      new w.Hyperlink("anchor", "anchor").addRun(new w.Run().addText("Hello!!"))
    );
    const p2 = new w.Paragraph()
      .addBookmarkStart(1, "anchor")
      .addRun(new w.Run().addText("World!!"))
      .pageBreakBefore(true)
      .addBookmarkEnd(1);
    const buffer = new w.Docx().addParagraph(p1).addParagraph(p2).build();

    writeFileSync("../output/js/anchor-hyperlink.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write external hyperlink", () => {
    const p1 = new w.Paragraph().addHyperlink(
      new w.Hyperlink("https://example.com", "external").addRun(
        new w.Run().addText("Hello!!")
      )
    );
    const buffer = new w.Docx().addParagraph(p1).build();

    writeFileSync("../output/js/external-hyperlink.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write dirty and disable auto items ToC", () => {
    const p1 = new w.Paragraph()
      .addRun(new w.Run().addText("Hello!!"))
      .pageBreakBefore(true)
      .style("Heading1");
    const style1 = new w.Style("Heading1", "paragraph").name("Heading 1");
    const p2 = new w.Paragraph()
      .addRun(new w.Run().addText("World"))
      .pageBreakBefore(true)
      .style("Heading2");
    const style2 = new w.Style("Heading2", "paragraph").name("Heading 2");
    const buffer = new w.Docx()
      .addTableOfContents(
        new w.TableOfContents().alias("Table of contents").dirty()
      )
      .addParagraph(p1)
      .addParagraph(p2)
      .addStyle(style1)
      .addStyle(style2)
      .build();
    writeFileSync("../output/js/toc_dirty_and_disable_auto_items.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write auto items ToC", () => {
    const p1 = new w.Paragraph()
      .addRun(new w.Run().addText("Hello!!"))
      .pageBreakBefore(true)
      .style("Heading1");
    const style1 = new w.Style("Heading1", "paragraph").name("Heading 1");
    const p2 = new w.Paragraph()
      .addRun(new w.Run().addText("World"))
      .pageBreakBefore(true)
      .style("Heading2");
    const style2 = new w.Style("Heading2", "paragraph").name("Heading 2");
    const buffer = new w.Docx()
      .addTableOfContents(
        new w.TableOfContents().alias("Table of contents").auto()
      )
      .addParagraph(p1)
      .addParagraph(p2)
      .addStyle(style1)
      .addStyle(style2)
      .build();
    writeFileSync("../output/js/toc_auto_items.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write ToC with items", () => {
    const p1 = new w.Paragraph()
      .addBookmarkStart(1, "_Toc00000000")
      .addRun(new w.Run().addText("Hello!!"))
      .addBookmarkEnd(1)
      .pageBreakBefore(true)
      .style("Heading1");
    const style1 = new w.Style("Heading1", "paragraph").name("Heading 1");
    const p2 = new w.Paragraph()
      .addBookmarkStart(2, "_Toc00000001")
      .addRun(new w.Run().addText("World"))
      .addBookmarkEnd(2)
      .pageBreakBefore(true)
      .style("Heading2");
    const style2 = new w.Style("Heading2", "paragraph").name("Heading 2");
    const buffer = new w.Docx()
      .addTableOfContents(
        new w.TableOfContents()
          .alias("Table of contents")
          .addItem(
            new w.TableOfContentsItem()
              .text("Hello!!")
              .level(1)
              .pageRef("2")
              .tocKey("_Toc00000000")
          )
          .addItem(
            new w.TableOfContentsItem()
              .text("World")
              .level(2)
              .pageRef("3")
              .tocKey("_Toc00000001")
          )
      )
      .addParagraph(p1)
      .addParagraph(p2)
      .addStyle(style1)
      .addStyle(style2)
      .build();
    writeFileSync("../output/js/toc_with_items.docx", buffer);
    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write pPrChange with inserted numbering", () => {
    const p = new w.Paragraph()
      .addRun(new w.Run().addText("Hello world!!"))
      .numbering(1, 0)
      .paragraphPropertyChange(
        new w.ParagraphPropertyChange().author("bokuweb")
      );

    const num = new w.Numbering(1, 0);
    const buffer = new w.Docx().addParagraph(p).addNumbering(num).build();

    writeFileSync(
      "../output/js/pprchange_with_inserted_numbering.docx",
      buffer
    );

    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write pPrChange with deleted numbering", () => {
    const p = new w.Paragraph()
      .addRun(new w.Run().addText("Hello world!!"))
      .paragraphPropertyChange(
        new w.ParagraphPropertyChange().author("bokuweb").numbering(1, 0)
      );

    const num = new w.Numbering(1, 0);
    const buffer = new w.Docx().addParagraph(p).addNumbering(num).build();

    writeFileSync("../output/js/pprchange_with_deleted_numbering.docx", buffer);

    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write paragraph delete", () => {
    const p1 = new w.Paragraph()
      .addRun(new w.Run().addText("Hello world!!"))
      .numbering(1, 0)
      .delete("bokuweb", "2021-12-23T18:16:00Z");
    const p2 = new w.Paragraph()
      .addRun(new w.Run().addText("Foo"))
      .numbering(1, 0);

    const num = new w.Numbering(1, 0);

    const buffer = new w.Docx()
      .addParagraph(p1)
      .addParagraph(p2)
      .addNumbering(num)
      .build();

    writeFileSync("../output/js/paragraph_delete.docx", buffer);

    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|numbering.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write inline image", () => {
    const buf = Buffer.from(encodedCat, "base64");
    const image = new w.Image(buf).size(320 * 9525, 240 * 9525);
    const p = new w.Paragraph().addRun(
      new w.Run().addText("Hello world!!").addImage(image)
    );
    const buffer = new w.Docx().addParagraph(p).build();

    writeFileSync("../output/js/inline_image.docx", buffer);

    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write inline jpeg image", () => {
    const buf = Buffer.from(require("./cat"), "base64");
    const image = new w.Image(buf).size(320 * 9525, 240 * 9525);
    const p = new w.Paragraph().addRun(
      new w.Run().addText("Hello world!!").addImage(image)
    );
    const buffer = new w.Docx().addParagraph(p).build();

    writeFileSync("../output/js/inline_image.docx", buffer);

    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write jpeg image with ins", () => {
    const buf = Buffer.from(require("./cat"), "base64");
    const image = new w.Image(buf).size(320 * 9525, 240 * 9525);
    const p = new w.Paragraph().addInsert(
      new w.Insert(new w.Run().addImage(image))
        .author("bokuweb")
        .date("2021-12-23T18:16:00Z")
    );

    const buffer = new w.Docx().addParagraph(p).build();

    writeFileSync("../output/js/image_with_ins.docx", buffer);

    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write jpeg image with del", () => {
    const buf = Buffer.from(require("./cat"), "base64");
    const image = new w.Image(buf).size(320 * 9525, 240 * 9525);
    const p = new w.Paragraph().addDelete(
      new w.Delete(new w.Run().addImage(image))
        .author("bokuweb")
        .date("2021-12-23T18:16:00Z")
    );

    const buffer = new w.Docx().addParagraph(p).build();

    writeFileSync("../output/js/image_with_del.docx", buffer);

    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
  });

  test("should write style", () => {
    const p = new w.Paragraph()
      .addRun(new w.Run().addText("Hello").style("Run"))
      .style("Heading1");
    const rStyle = new w.Style("Run", "character").name("Run test").bold();
    const pStyle = new w.Style("Heading1", "paragraph")
      .name("Heading 1")
      .align("center");
    const tStyle = new w.Style("Table", "table")
      .name("Table 1")
      .tableAlign("center")
      .tableIndent(200);

    const table = new w.Table()
      .addRow(
        new w.TableRow().addCell(
          new w.TableCell().addParagraph(
            new w.Paragraph().addRun(new w.Run().addText("Hello"))
          )
        )
      )
      .style("Table");

    const buffer = new w.Docx()
      .addStyle(pStyle)
      .addStyle(rStyle)
      .addStyle(tStyle)
      .addParagraph(p)
      .addTable(table)
      .build();

    const z = new Zip(Buffer.from(buffer));
    for (const e of z.getEntries()) {
      if (e.entryName.match(/document.xml|styles.xml/)) {
        expect(z.readAsText(e)).toMatchSnapshot();
      }
    }
    writeFileSync("../output/js/style.docx", buffer);
  });
});
