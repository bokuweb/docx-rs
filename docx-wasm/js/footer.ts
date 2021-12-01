import { Paragraph } from "./paragraph";
import { Table } from "./table";

export class Footer {
  hasNumberings = false;
  children: (Paragraph | Table)[] = [];

  addParagraph(p: Paragraph) {
    if (p.hasNumberings) {
      this.hasNumberings = true;
    }
    this.children.push(p);
    return this;
  }

  addTable(t: Table) {
    if (t.hasNumberings) {
      this.hasNumberings = true;
    }
    this.children.push(t);
    return this;
  }
}
