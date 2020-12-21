import { Paragraph } from "./paragraph";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { DeleteText } from "./delete-text";
import { Table } from "./table";
import { TableCell } from "./table-cell";
import { BorderType } from "./table-cell-border";
import { Run, RunFonts } from "./run";
import { Text } from "./text";
import { Tab } from "./tab";
import { Break } from "./break";
import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";
import { AbstractNumbering } from "./abstract-numbering";
import { Numbering } from "./numbering";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Settings } from "./settings";
import { Styles } from "./styles";
import { SectionProperty, PageMargin } from "./section-property";
import { DocxJSON } from "./json";

import * as wasm from "./pkg";
import { Level } from "./level";

const convertBorderType = (t: BorderType) => {
  switch (t) {
    case "nil":
      return wasm.BorderType.Nil;
    case "none":
      return wasm.BorderType.None;
    case "single":
      return wasm.BorderType.Single;
    case "thick":
      return wasm.BorderType.Thick;
    case "double":
      return wasm.BorderType.Double;
    case "dotted":
      return wasm.BorderType.Dotted;
    case "dashed":
      return wasm.BorderType.Dashed;
    case "dotDash":
      return wasm.BorderType.DotDash;
    case "dotDotDash":
      return wasm.BorderType.DotDotDash;
    case "triple":
      return wasm.BorderType.Triple;
    default:
      return wasm.BorderType.Single;
  }
};

export class Docx {
  children: (Paragraph | Table | BookmarkStart | BookmarkEnd)[] = [];
  hasNumberings = false;
  abstractNumberings: AbstractNumbering[] = [];
  numberings: Numbering[] = [];
  settings: Settings = new Settings();
  sectionProperty: SectionProperty = new SectionProperty();
  styles = new Styles();

  addParagraph(p: Paragraph) {
    if (p.hasNumberings) {
      this.hasNumberings = true;
    }
    this.children.push(p);
    return this;
  }

  addBookmarkStart(id: number, name: string) {
    this.children.push(new BookmarkStart(id, name));
    return this;
  }

  addBookmarkEnd(id: number) {
    this.children.push(new BookmarkEnd(id));
    return this;
  }

  addTable(t: Table) {
    if (t.hasNumberings) {
      this.hasNumberings = true;
    }
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

  docId(id: string) {
    this.settings.docId(id);
    return this;
  }

  addDocVar(name: string, val: string) {
    this.settings.addDocVar(name, val);
    return this;
  }

  pageSize(w: number, h: number) {
    this.sectionProperty.pageSize(w, h);
    return this;
  }

  pageMargin(margin: Partial<PageMargin>) {
    this.sectionProperty.pageMargin(margin);
    return this;
  }

  defaultSize(size: number) {
    this.styles.defaultSize(size);
    return this;
  }

  defaultFonts(fonts: RunFonts) {
    this.styles.defaultFonts(fonts);
    return this;
  }

  defaultSpacing(spacing: number) {
    this.styles.defaultSpacing(spacing);
    return this;
  }

  buildRunFonts = (fonts: RunFonts | undefined) => {
    let f = wasm.createRunFonts();
    if (fonts?._ascii) {
      f = f.ascii(fonts._ascii);
    }
    if (fonts?._hiAnsi) {
      f = f.hi_ansi(fonts._hiAnsi);
    }
    if (fonts?._cs) {
      f = f.cs(fonts._cs);
    }
    if (fonts?._eastAsia) {
      f = f.east_asia(fonts._eastAsia);
    }
    return f;
  };

  buildRun(r: Run) {
    let run = wasm.createRun();
    r.children.forEach((child) => {
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

    if (r.property.spacing != null) {
      run = run.spacing(r.property.spacing);
    }

    const fonts = this.buildRunFonts(r.property.fonts);
    run = run.fonts(fonts);

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
    c.children.forEach((child) => {
      if (child instanceof Paragraph) {
        comment = comment.add_paragraph(this.buildParagraph(child));
      } else if (child instanceof Table) {
        // TODO:
      }
    });
    if (c._author) {
      comment = comment.author(c._author);
    }
    if (c._date) {
      comment = comment.date(c._date);
    }
    if (c._parentCommentId) {
      comment = comment.parent_comment_id(c._parentCommentId);
    }
    return comment;
  }

  buildParagraph(p: Paragraph) {
    let paragraph = wasm.createParagraph();
    p.children.forEach((child) => {
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
      case "distribute": {
        paragraph = paragraph.align(wasm.AlignmentType.Distribute);
        break;
      }
      case "both": {
        paragraph = paragraph.align(wasm.AlignmentType.Both);
        break;
      }
      case "end": {
        paragraph = paragraph.align(wasm.AlignmentType.End);
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

    if (p.property.runProperty.bold) {
      paragraph = paragraph.bold();
    }

    if (p.property.lineHeight) {
      paragraph = paragraph.line_height(p.property.lineHeight);
    }

    if (p.property.runProperty.italic) {
      paragraph = paragraph.italic();
    }

    if (p.property.runProperty.size) {
      paragraph = paragraph.size(p.property.runProperty.size);
    }

    if (p.property.runProperty.fonts) {
      let f = wasm.createRunFonts();
      if (p.property.runProperty.fonts._ascii) {
        f = f.ascii(p.property.runProperty.fonts._ascii);
      }
      if (p.property.runProperty.fonts._hiAnsi) {
        f = f.hi_ansi(p.property.runProperty.fonts._hiAnsi);
      }
      if (p.property.runProperty.fonts._cs) {
        f = f.cs(p.property.runProperty.fonts._cs);
      }
      if (p.property.runProperty.fonts._eastAsia) {
        f = f.east_asia(p.property.runProperty.fonts._eastAsia);
      }
      paragraph = paragraph.fonts(f);
    }

    return paragraph;
  }

  buildTable(t: Table) {
    let table = wasm.createTable();
    t.rows.forEach((r) => {
      let row = wasm.createTableRow();
      r.cells.forEach((c) => {
        const cell = this.buildCell(c);
        row = row.add_cell(cell);
      });
      if (r.height) {
        row = row.row_height(r.height);
      }
      table = table.add_row(row);
    });
    table = table.set_grid(new Uint32Array(t.grid));
    table = table.indent(t.property.indent || 0);

    if (t.property.cellMargins) {
      const { top, right, bottom, left } = t.property.cellMargins;
      table = table.set_cell_margins(top, right, bottom, left);
    }

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
    c.children.forEach((p) => {
      const paragraph = this.buildParagraph(p);
      cell = cell.add_paragraph(paragraph);
    });

    if (c.property.verticalMerge === "continue") {
      cell = cell.vertical_merge(wasm.VMergeType.Continue);
    } else if (c.property.verticalMerge === "restart") {
      cell = cell.vertical_merge(wasm.VMergeType.Restart);
    }

    switch (c.property.verticalAlign) {
      case "top": {
        cell = cell.vertical_align(wasm.VAlignType.Top);
        break;
      }
      case "center": {
        cell = cell.vertical_align(wasm.VAlignType.Center);
        break;
      }
      case "bottom": {
        cell = cell.vertical_align(wasm.VAlignType.Bottom);
        break;
      }
    }

    if (typeof c.property.gridSpan !== "undefined") {
      cell = cell.grid_span(c.property.gridSpan);
    }

    if (typeof c.property.width !== "undefined") {
      cell = cell.width(c.property.width);
    }
    if (typeof c.property.borders !== "undefined") {
      if (c.property.borders.top) {
        const border = wasm
          .createTableCellBorder(wasm.BorderPosition.Top)
          .size(c.property.borders.top._size)
          .color(c.property.borders.top._color)
          .border_type(convertBorderType(c.property.borders.top._border_type));
        cell = cell.set_border(border);
      }

      if (c.property.borders.right) {
        const border = wasm
          .createTableCellBorder(wasm.BorderPosition.Right)
          .size(c.property.borders.right._size)
          .color(c.property.borders.right._color)
          .border_type(
            convertBorderType(c.property.borders.right._border_type)
          );
        cell = cell.set_border(border);
      }

      if (c.property.borders.bottom) {
        const border = wasm
          .createTableCellBorder(wasm.BorderPosition.Bottom)
          .size(c.property.borders.bottom._size)
          .color(c.property.borders.bottom._color)
          .border_type(
            convertBorderType(c.property.borders.bottom._border_type)
          );
        cell = cell.set_border(border);
      }

      if (c.property.borders.left) {
        const border = wasm
          .createTableCellBorder(wasm.BorderPosition.Left)
          .size(c.property.borders.left._size)
          .color(c.property.borders.left._color)
          .border_type(convertBorderType(c.property.borders.left._border_type));
        cell = cell.set_border(border);
      }

      if (c.property.borders.insideH) {
        const border = wasm
          .createTableCellBorder(wasm.BorderPosition.InsideH)
          .size(c.property.borders.insideH._size)
          .color(c.property.borders.insideH._color)
          .border_type(
            convertBorderType(c.property.borders.insideH._border_type)
          );
        cell = cell.set_border(border);
      }

      if (c.property.borders.insideV) {
        const border = wasm
          .createTableCellBorder(wasm.BorderPosition.InsideV)
          .size(c.property.borders.insideV._size)
          .color(c.property.borders.insideV._color)
          .border_type(
            convertBorderType(c.property.borders.insideV._border_type)
          );
        cell = cell.set_border(border);
      }
    }

    return cell;
  }

  buildLevel(l: Level) {
    let level = wasm.createLevel(l.id, l.start, l.format, l.text, l.jc);

    if (l.levelSuffix === "nothing") {
      level = level.suffix(wasm.LevelSuffixType.Nothing);
    } else if (l.levelSuffix === "space") {
      level = level.suffix(wasm.LevelSuffixType.Space);
    } else {
      level = level.suffix(wasm.LevelSuffixType.Tab);
    }

    if (l.runProperty.bold) {
      level = level.bold();
    }

    if (l.runProperty.italic) {
      level = level.italic();
    }

    if (l.runProperty.size) {
      level = level.size(l.runProperty.size);
    }

    if (l.runProperty.fonts) {
      let f = wasm.createRunFonts();
      if (l.runProperty.fonts._ascii) {
        f = f.ascii(l.runProperty.fonts._ascii);
      }
      if (l.runProperty.fonts._hiAnsi) {
        f = f.hi_ansi(l.runProperty.fonts._hiAnsi);
      }
      if (l.runProperty.fonts._cs) {
        f = f.cs(l.runProperty.fonts._cs);
      }
      if (l.runProperty.fonts._eastAsia) {
        f = f.east_asia(l.runProperty.fonts._eastAsia);
      }
      level = level.fonts(f);
    }

    if (l.paragraphProperty.indent) {
      let kind;
      if (l.paragraphProperty.indent.specialIndentKind === "firstLine") {
        kind = wasm.SpecialIndentKind.FirstLine;
      } else if (l.paragraphProperty.indent.specialIndentKind === "hanging") {
        kind = wasm.SpecialIndentKind.Hanging;
      }
      level = level.indent(
        l.paragraphProperty.indent.left,
        kind,
        l.paragraphProperty.indent.specialIndentSize
      );
    }
    return level;
  }

  build(opts?: { json?: boolean }): { buffer: Uint8Array; json?: string } {
    let docx = wasm.createDocx();

    this.children.forEach((child) => {
      if (child instanceof Paragraph) {
        let p = this.buildParagraph(child);
        docx = docx.add_paragraph(p);
      } else if (child instanceof Table) {
        let t = this.buildTable(child);
        docx = docx.add_table(t);
      } else if (child instanceof BookmarkStart) {
        docx = docx.add_bookmark_start(child.id, child.name);
      } else if (child instanceof BookmarkEnd) {
        docx = docx.add_bookmark_end(child.id);
      }
    });

    this.abstractNumberings.forEach((n) => {
      let num = wasm.createAbstractNumbering(n.id);
      n.levels.forEach((l) => {
        const level = this.buildLevel(l);
        num = num.add_level(level);
      });
      docx = docx.add_abstract_numbering(num);
    });

    this.numberings.forEach((n) => {
      let num = wasm.createNumbering(n.id, n.abstractNumId);
      n.overrides.forEach((o) => {
        let levelOverride = wasm.createLevelOverride(o.level);
        if (o.startOverride !== null) {
          levelOverride = levelOverride.start(o.startOverride);
        }
        if (o.levelOverride !== null) {
          let level = wasm.createLevel(
            o.levelOverride.level,
            o.levelOverride.start,
            o.levelOverride.format,
            o.levelOverride.text,
            o.levelOverride.jc
          );
          levelOverride = levelOverride.level(level);
        }
        num = num.add_override(levelOverride);
      });
      docx = docx.add_numbering(num);
    });

    if (this.settings._docId) {
      docx = docx.doc_id(this.settings._docId);
    }

    this.settings._docVars.forEach((v) => {
      docx = docx.add_doc_var(v.name, v.val);
    });

    if (this.sectionProperty._pageMargin) {
      const {
        top,
        left,
        right,
        bottom,
        header,
        footer,
        gutter,
      } = this.sectionProperty._pageMargin;
      const margin = wasm
        .createPageMargin()
        .top(top)
        .left(left)
        .right(right)
        .bottom(bottom)
        .header(header)
        .footer(footer)
        .gutter(gutter);
      docx = docx.page_margin(margin);
    }

    if (this.sectionProperty._pageSize) {
      const { w, h } = this.sectionProperty._pageSize;
      docx = docx.page_size(w, h);
    }

    if (this.styles?.docDefaults) {
      if (this.styles.docDefaults.runProperty?.fonts) {
        const fonts = this.buildRunFonts(
          this.styles.docDefaults.runProperty.fonts
        );
        docx = docx.default_fonts(fonts);
      }

      if (this.styles.docDefaults.runProperty?.size) {
        docx = docx.default_size(this.styles.docDefaults.runProperty.size);
      }

      if (this.styles.docDefaults.runProperty?.spacing) {
        docx = docx.default_spacing(
          this.styles.docDefaults.runProperty.spacing
        );
      }
    }

    const json = opts?.json ? docx.json() : undefined;
    const buffer = docx.build(this.hasNumberings);
    docx.free();
    return { buffer, json };
  }
}

export const readDocx = (buf: Uint8Array) => {
  return JSON.parse(wasm.readDocx(buf)) as DocxJSON;
};

export * from "./paragraph";
export * from "./insert";
export * from "./delete";
export * from "./table";
export * from "./table-cell";
export * from "./table-cell-border";
export * from "./table-cell-borders";
export * from "./table-row";
export * from "./run";
export * from "./text";
export * from "./style";
export * from "./styles";
export * from "./comment";
export * from "./comment-end";
export * from "./numbering";
export * from "./abstract-numbering";
export * from "./bookmark-start";
export * from "./bookmark-end";
export * from "./break";
export * from "./delete-text";
export * from "./level";
export * from "./tab";
export * from "./json";
