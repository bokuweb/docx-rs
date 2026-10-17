const w = require("../dist/node");
const Zip = require("adm-zip");

describe("table row cantSplit reader", () => {
  test.each([
    ["<w:cantSplit/>", true],
    ['<w:cantSplit w:val="1"/>', true],
    ['<w:cantSplit w:val="true"/>', true],
    ['<w:cantSplit w:val="on"/>', true],
    ['<w:cantSplit w:val="0"/>', false],
    ['<w:cantSplit w:val="false"/>', false],
    ['<w:cantSplit w:val="off"/>', false],
    ["", false],
  ])("reads %s as an enabled constraint only when true", (property, enabled) => {
    const zip = new Zip(Buffer.from(new w.Docx().build()));
    zip.updateFile(
      "word/document.xml",
      Buffer.from(
        `<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:tbl><w:tr><w:trPr>${property}</w:trPr><w:tc><w:p><w:r><w:t>row content</w:t></w:r></w:p></w:tc></w:tr></w:tbl></w:body></w:document>`
      )
    );
    const json = w.readDocx(zip.toBuffer());
    const row = json.document.children[0].data.rows[0].data;
    expect(row.property.cantSplit).toBe(enabled ? "cantSplit" : undefined);
    expect(row.cells).toHaveLength(1);
  });
});
