import { Paragraph } from "./paragraph";
import { Table } from "./table";
import { Run } from "./run";
import { Text } from "./text";
import { Numbering } from "./numbering";

const rust = import("../pkg");

export class Docx {
  children: (Paragraph | Table)[] = [];
  numberings: Numbering[];

  addParagraph(p: Paragraph) {
    this.children.push(p);
    return this;
  }

  addTable(t: Table) {
    this.children.push(t);
    return this;
  }

  addNumbering(num: Numbering) {
    this.numberings.push(num);
    return this;
  }

  build() {
    rust.then(m => {
      let docx = m.createDocx();
      this.children.forEach(child => {
        if (child instanceof Paragraph) {
          let p = m.createParagraph();
          child.children.forEach(c => {
            let r = m.createRun();
            if (c instanceof Run) {
              c.children.forEach(a => {
                if (a instanceof Text) {
                  r = r.add_text(a.text);
                }
              });
            }
            p = p.add_run(r);
          });
          docx = docx.add_paragraph(p);
        }
      });
      console.log("aaa", docx.build());
      docx.free();
    });
  }
}

const d = new Docx().addParagraph(
  new Paragraph().addRun(new Run().addText(new Text("Hello")))
);

d.build();
