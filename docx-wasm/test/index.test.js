const w = require("../dist/node");
const { readFileSync } = require("fs");

describe("reader", () => {
  test("should read lvlOverride docx", () => {
    const buf = readFileSync("../fixtures/lvl_override/override.docx");
    const json = w.readDocx(buf);
    console.log(json);
    expect(json).toMatchSnapshot();
  });
});
