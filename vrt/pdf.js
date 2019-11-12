const libre = require("libreoffice-convert");
const path = require("path");
const fs = require("fs");

const extend = "png";

module.exports = (docxPath, outputDir) =>
  new Promise((resolve, reject) => {
    const filename = path.basename(docxPath, ".docx");
    const docxFile = fs.readFileSync(docxPath);
    libre.convert(docxFile, extend, undefined, async (err, done) => {
      if (err) {
        reject(err);
      }
      try {
        fs.mkdirSync(outputDir, { recursive: true });
      } catch (e) {
        if (e.code !== "EEXIST") {
          reject(e);
        }
      }
      fs.writeFileSync(path.join(outputDir, `${filename}.${extend}`), done);
      resolve();
    });
  });
