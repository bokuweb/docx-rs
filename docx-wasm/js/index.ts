import { Paragraph } from "./paragraph";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { DeleteText } from "./delete-text";
import { Table } from "./table";
import { TableCell } from "./table-cell";
import { Run } from "./run";
import { Text } from "./text";
import { Tab } from "./tab";
import { Break } from "./break";
import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";
import { AbstractNumbering } from "./abstract-numbering";
import { Numbering } from "./numbering";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";

import * as wasm from "../pkg";

export class Docx {
  children: (Paragraph | Table)[] = [];
  abstractNumberings: AbstractNumbering[] = [];
  numberings: Numbering[] = [];

  addParagraph(p: Paragraph) {
    this.children.push(p);
    return this;
  }

  addTable(t: Table) {
    this.children.push(t);
    return this;
  }

  addAbstractNumbering(num: AbstractNumbering) {
    this.abstractNumberings.push(num);
    return this;
  }

  addNumbering(num: Numbering) {
    this.numberings.push(num);
    return this;
  }

  buildRun(r: Run) {
    let run = wasm.createRun();
    r.children.forEach(child => {
      if (child instanceof Text) {
        run = run.add_text(child.text);
      } else if (child instanceof DeleteText) {
        run = run.add_delete_text(child.text);
      } else if (child instanceof Tab) {
        run = run.add_tab();
      } else if (child instanceof Break) {
        if (child.type === "column") {
          run = run.add_break(wasm.BreakType.Column);
        } else if (child.type === "page") {
          run = run.add_break(wasm.BreakType.Page);
        } else if (child.type === "textWrapping") {
          run = run.add_break(wasm.BreakType.TextWrapping);
        }
      }
    });

    if (typeof r.property.size !== "undefined") {
      run = run.size(r.property.size);
    }

    if (r.property.color) {
      run = run.color(r.property.color);
    }

    if (r.property.highlight) {
      run = run.highlight(r.property.highlight);
    }

    if (r.property.bold) {
      run = run.bold();
    }

    if (r.property.italic) {
      run = run.italic();
    }

    if (r.property.underline) {
      run = run.underline(r.property.underline);
    }

    if (r.property.vanish) {
      run = run.vanish();
    }

    return run;
  }

  buildInsert(i: Insert) {
    const run = this.buildRun(i.run);
    let insert = wasm.createInsert(run);
    if (i._author) {
      insert = insert.author(i._author);
    }
    if (i._date) {
      insert = insert.date(i._date);
    }
    return insert;
  }

  buildDelete(d: Delete) {
    const run = this.buildRun(d.run);
    let del = wasm.createDelete(run);
    if (d._author) {
      del = del.author(d._author);
    }
    if (d._date) {
      del = del.date(d._date);
    }
    return del;
  }

  buildComment(c: Comment) {
    let comment = wasm.createComment(c.id);
    if (c._paragraph) {
      comment = comment.paragraph(this.buildParagraph(c._paragraph));
    }
    if (c._author) {
      comment = comment.author(c._author);
    }
    if (c._date) {
      comment = comment.date(c._date);
    }
    return comment;
  }

  buildParagraph(p: Paragraph) {
    let paragraph = wasm.createParagraph();
    p.children.forEach(child => {
      if (child instanceof Run) {
        const run = this.buildRun(child);
        paragraph = paragraph.add_run(run);
      } else if (child instanceof Insert) {
        const insert = this.buildInsert(child);
        paragraph = paragraph.add_insert(insert);
      } else if (child instanceof Delete) {
        const del = this.buildDelete(child);
        paragraph = paragraph.add_delete(del);
      } else if (child instanceof BookmarkStart) {
        paragraph = paragraph.add_bookmark_start(child.id, child.name);
      } else if (child instanceof BookmarkEnd) {
        paragraph = paragraph.add_bookmark_end(child.id);
      } else if (child instanceof Comment) {
        const comment = this.buildComment(child);
        paragraph = paragraph.add_comment_start(comment);
      } else if (child instanceof CommentEnd) {
        paragraph = paragraph.add_comment_end(child.id);
      }
    });

    switch (p.property.align) {
      case "center": {
        paragraph = paragraph.align(wasm.AlignmentType.Center);
        break;
      }
      case "right": {
        paragraph = paragraph.align(wasm.AlignmentType.Right);
        break;
      }
      case "justified": {
        paragraph = paragraph.align(wasm.AlignmentType.Justified);
        break;
      }
      case "left": {
        paragraph = paragraph.align(wasm.AlignmentType.Left);
        break;
      }
    }

    if (typeof p.property.indent !== "undefined") {
      const { indent } = p.property;
      let kind;
      switch (p.property.indent.specialIndentKind) {
        case "firstLine": {
          kind = wasm.SpecialIndentKind.FirstLine;
          break;
        }
        case "hanging": {
          kind = wasm.SpecialIndentKind.Hanging;
          break;
        }
      }
      paragraph = paragraph.indent(indent.left, kind, indent.specialIndentSize);
    }

    if (typeof p.property.numbering !== "undefined") {
      const { numbering } = p.property;
      paragraph = paragraph.numbering(numbering.id, numbering.level);
    }

    if (typeof p.property.styleId !== "undefined") {
      paragraph = paragraph.style(p.property.styleId);
    }

    return paragraph;
  }

  buildTable(t: Table) {
    let table = wasm.createTable();
    t.rows.forEach(r => {
      let row = wasm.createTableRow();
      r.cells.forEach(c => {
        const cell = this.buildCell(c);
        row = row.add_cell(cell);
      });
      table = table.add_row(row);
    });
    table = table.set_grid(new Uint32Array(t.grid));

    switch (t.property.align) {
      case "center": {
        table = table.align(wasm.TableAlignmentType.Center);
        break;
      }
      case "right": {
        table = table.align(wasm.TableAlignmentType.Right);
        break;
      }
      case "left": {
        table = table.align(wasm.TableAlignmentType.Left);
        break;
      }
    }
    return table;
  }

  buildCell(c: TableCell) {
    let cell = wasm.createTableCell();
    c.children.forEach(p => {
      const paragraph = this.buildParagraph(p);
      cell = cell.add_paragraph(paragraph);
    });

    if (c.property.verticalMerge === "continue") {
      cell = cell.vertical_merge(wasm.VMergeType.Continue);
    } else if (c.property.verticalMerge === "restart") {
      cell = cell.vertical_merge(wasm.VMergeType.Restart);
    }

    if (typeof c.property.gridSpan !== "undefined") {
      cell = cell.grid_span(c.property.gridSpan);
    }

    if (typeof c.property.width !== "undefined") {
      cell = cell.width(c.property.width);
    }
    return cell;
  }

  build() {
    let docx = wasm.createDocx();
    this.children.forEach(child => {
      if (child instanceof Paragraph) {
        let p = this.buildParagraph(child);
        docx = docx.add_paragraph(p);
      } else if (child instanceof Table) {
        let t = this.buildTable(child);
        docx = docx.add_table(t);
      }
    });

    this.abstractNumberings.forEach(n => {
      let num = wasm.createAbstractNumbering(n.id);
      n.levels.forEach(l => {
        const level = wasm.createLevel(l.id, l.start, l.format, l.text, l.jc);
        num = num.add_level(level);
      });
      docx = docx.add_abstract_numbering(num);
    });

    this.numberings.forEach(n => {
      let num = wasm.createNumbering(n.id, n.abstractNumId);
      docx = docx.add_numbering(num);
    });

    const buf = docx.build();
    docx.free();
    return buf;
  }
}

export const readDocx = (buf: Uint8Array) => {
  return wasm.readDocx(buf);
};

export * from "./paragraph";
export * from "./insert";
export * from "./delete";
export * from "./table";
export * from "./table-cell";
export * from "./table-row";
export * from "./run";
export * from "./text";
export * from "./comment";
export * from "./comment-end";
export * from "./numbering";
export * from "./bookmark-start";
export * from "./bookmark-end";
export * from "./break";
export * from "./delete-text";
export * from "./level";
export * from "./tab";
