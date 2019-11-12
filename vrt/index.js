const glob = require("glob");
const path = require("path");
const createPDF = require("./pdf");

glob(
  path.join(__dirname, "..", "./docx-core/tests/output/**/*.docx"),
  {},
  async (err, files) => {
    for await (file of files) {
      await createPDF(file, path.join(__dirname, "./screenshot/actual"));
    }
  }
);
