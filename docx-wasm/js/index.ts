import { Paragraph } from "./paragraph";
import { ParagraphProperty } from "./paragraph-property";
import { Table } from "./table";
import { TableOfContents } from "./table-of-contents";
import { RunFonts } from "./run";
import { AbstractNumbering } from "./abstract-numbering";
import { Numbering } from "./numbering";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { CharacterSpacingValues, Settings } from "./settings";
import { DocProps } from "./doc-props";
import { Style } from "./style";
import { Styles } from "./styles";
import { WebExtension } from "./webextension";
import { Footer } from "./footer";
import { Header } from "./header";
import { build } from "./builder";

import {
  SectionProperty,
  PageMargin,
  PageOrientationType,
} from "./section-property";
import { DocGridType, DocxJSON } from "./json";

import * as wasm from "./pkg";
import { Level } from "./level";

export class Docx {
  children: (
    | Paragraph
    | Table
    | BookmarkStart
    | BookmarkEnd
    | TableOfContents
  )[] = [];

  hasNumberings = false;
  abstractNumberings: AbstractNumbering[] = [];
  numberings: Numbering[] = [];
  settings: Settings = new Settings();
  docProps: DocProps = new DocProps();
  sectionProperty: SectionProperty = new SectionProperty();
  _taskpanes: boolean = false;
  webextensions: WebExtension[] = [];
  customItems: { id: string; xml: string }[] = [];
  styles = new Styles();

  addTableOfContents(t: TableOfContents) {
    this.children.push(t);
    return this;
  }

  addStyle(s: Style) {
    this.styles.styles.push(s);
    return this;
  }

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

  defaultTabStop(stop: number) {
    this.settings.defaultTabStop(stop);
    return this;
  }

  createdAt(date: string) {
    this.docProps.createdAt(date);
    return this;
  }

  customProperty(name: string, item: string) {
    this.docProps.customProperty(name, item);
    return this;
  }

  updatedAt(date: string) {
    this.docProps.updatedAt(date);
    return this;
  }

  addDocVar(name: string, val: string) {
    this.settings.addDocVar(name, val);
    return this;
  }

  header(f: Header) {
    this.sectionProperty._header = f;
    return this;
  }

  firstHeader(h: Header) {
    this.sectionProperty._firstHeader = h;
    return this;
  }

  evenHeader(h: Header) {
    this.sectionProperty._evenHeader = h;
    return this;
  }

  footer(f: Footer) {
    this.sectionProperty._footer = f;
    return this;
  }

  firstFooter(f: Footer) {
    this.sectionProperty._firstFooter = f;
    return this;
  }

  evenFooter(f: Footer) {
    this.sectionProperty._evenFooter = f;
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

  pageOrientation(o: PageOrientationType) {
    this.sectionProperty.pageOrientation(o);
    return this;
  }

  docGrid(type: DocGridType, linePitch?: number, charSpace?: number) {
    this.sectionProperty.docGrid(type, linePitch, charSpace);
    return this;
  }

  adjustLineHeightInTable() {
    this.settings.adjustLineHeightInTable();
    return this;
  }

  characterSpacingControl(v: CharacterSpacingValues) {
    this.settings._characterSpacingControl = v;
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

  defaultCharacterSpacing(spacing: number) {
    this.styles.defaultCharacterSpacing(spacing);
    return this;
  }

  taskpanes() {
    this._taskpanes = true;
    return this;
  }

  webextension(e: WebExtension) {
    this.webextensions.push(e);
    return this;
  }

  addCustomItem(id: string, xml: string) {
    this.customItems.push({ id, xml });
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

  buildLineSpacing(p: ParagraphProperty): wasm.LineSpacing | null {
    const { lineSpacing } = p;
    if (lineSpacing == null) return null;
    let kind;
    switch (lineSpacing._lineRule) {
      case "atLeast": {
        kind = wasm.LineSpacingType.AtLeast;
        break;
      }
      case "auto": {
        kind = wasm.LineSpacingType.Auto;
        break;
      }
      case "exact": {
        kind = wasm.LineSpacingType.Exact;
        break;
      }
    }
    let spacing = wasm.createLineSpacing();
    if (lineSpacing._before != null) {
      spacing = spacing.before(lineSpacing._before);
    }

    if (lineSpacing._after != null) {
      spacing = spacing.after(lineSpacing._after);
    }

    if (lineSpacing._beforeLines != null) {
      spacing = spacing.before_lines(lineSpacing._beforeLines);
    }

    if (lineSpacing._afterLines != null) {
      spacing = spacing.after_lines(lineSpacing._afterLines);
    }

    if (lineSpacing._line != null) {
      spacing = spacing.line(lineSpacing._line);
    }

    if (kind != null) {
      spacing = spacing.line_rule(kind);
    }
    return spacing;
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

  createDocx(): wasm.Docx {
    let docx = wasm.createDocx();

    this.children.forEach((child) => {
      if (child instanceof Paragraph) {
        docx = docx.add_paragraph(build(child));
      } else if (child instanceof Table) {
        let t = child.build();
        docx = docx.add_table(t);
      } else if (child instanceof BookmarkStart) {
        docx = docx.add_bookmark_start(child.id, child.name);
      } else if (child instanceof BookmarkEnd) {
        docx = docx.add_bookmark_end(child.id);
      } else if (child instanceof TableOfContents) {
        docx = docx.add_table_of_contents(child.buildWasmObject());
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

    if (this.settings._adjustLineHeightInTable) {
      docx = docx.set_adjust_line_height_in_table();
    }

    if (this.settings._characterSpacingControl) {
      if (this.settings._characterSpacingControl === "compressPunctuation") {
        docx = docx.character_spacing_control(
          wasm.CharacterSpacingValues.CompressPunctuation
        );
      } else if (this.settings._characterSpacingControl === "doNotCompress") {
        docx = docx.character_spacing_control(
          wasm.CharacterSpacingValues.DoNotCompress
        );
      } else if (
        this.settings._characterSpacingControl ===
        "compressPunctuationAndJapaneseKana"
      ) {
        docx = docx.character_spacing_control(
          wasm.CharacterSpacingValues.CompressPunctuationAndJapaneseKana
        );
      }
    }

    docx = docx.default_tab_stop(this.settings._defaultTabStop);

    this.settings._docVars.forEach((v) => {
      docx = docx.add_doc_var(v.name, v.val);
    });

    if (this.sectionProperty._header) {
      let header = wasm.createHeader();
      this.sectionProperty._header.children.forEach((c) => {
        if (c instanceof Paragraph) {
          header = header.add_paragraph(build(c));
        } else {
          header = header.add_table(c.build());
        }
      });
      docx = docx.header(header);
    }

    if (this.sectionProperty._firstHeader) {
      let header = wasm.createHeader();
      this.sectionProperty._firstHeader.children.forEach((c) => {
        if (c instanceof Paragraph) {
          header = header.add_paragraph(build(c));
        } else {
          header = header.add_table(c.build());
        }
      });
      docx = docx.first_header(header);
    }

    if (this.sectionProperty._evenHeader) {
      let header = wasm.createHeader();
      this.sectionProperty._evenHeader.children.forEach((c) => {
        if (c instanceof Paragraph) {
          header = header.add_paragraph(build(c));
        } else {
          header = header.add_table(c.build());
        }
      });
      docx = docx.even_header(header);
    }

    if (this.sectionProperty._footer) {
      let footer = wasm.createFooter();
      this.sectionProperty._footer.children.forEach((c) => {
        if (c instanceof Paragraph) {
          footer = footer.add_paragraph(build(c));
        } else {
          footer = footer.add_table(c.build());
        }
      });
      docx = docx.footer(footer);
    }

    if (this.sectionProperty._firstFooter) {
      let footer = wasm.createFooter();
      this.sectionProperty._firstFooter.children.forEach((c) => {
        if (c instanceof Paragraph) {
          footer = footer.add_paragraph(build(c));
        } else {
          footer = footer.add_table(c.build());
        }
      });
      docx = docx.first_footer(footer);
    }

    if (this.sectionProperty._evenFooter) {
      let footer = wasm.createFooter();
      this.sectionProperty._evenFooter.children.forEach((c) => {
        if (c instanceof Paragraph) {
          footer = footer.add_paragraph(build(c));
        } else {
          footer = footer.add_table(c.build());
        }
      });
      docx = docx.even_footer(footer);
    }

    if (this.sectionProperty._pageMargin) {
      const { top, left, right, bottom, header, footer, gutter } =
        this.sectionProperty._pageMargin;
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
      const { w, h, orient } = this.sectionProperty._pageSize;
      docx = docx.page_size(w, h);
      switch (orient) {
        case "landscape":
          docx = docx.page_orient(wasm.PageOrientationType.Landscape);
          break;
        case "portrait":
          docx = docx.page_orient(wasm.PageOrientationType.Portrait);
          break;
      }
    }

    if (this.sectionProperty._docGrid) {
      const { gridType, charSpace, linePitch } = this.sectionProperty._docGrid;
      let type = wasm.DocGridType.Default;
      switch (gridType) {
        case "lines":
          type = wasm.DocGridType.Lines;
          break;
        case "linesAndChars":
          type = wasm.DocGridType.LinesAndChars;
          break;
        case "snapToChars":
          type = wasm.DocGridType.SnapToChars;
          break;
        case "default":
          break;
      }
      docx = docx.doc_grid(type, linePitch, charSpace);
    }

    for (const s of this.styles?.styles) {
      docx = docx.add_style(s.buildWasmObject());
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

      if (this.styles.docDefaults.runProperty?.characterSpacing) {
        docx = docx.default_spacing(
          this.styles.docDefaults.runProperty.characterSpacing
        );
      }
    }

    if (this.docProps._createdAt) {
      docx = docx.created_at(this.docProps._createdAt);
    }

    if (this.docProps._updatedAt) {
      docx = docx.updated_at(this.docProps._updatedAt);
    }

    Object.entries(this.docProps._customProperties).forEach(([key, item]) => {
      docx = docx.custom_property(key, item);
    });

    if (this.docProps._updatedAt) {
      docx = docx.updated_at(this.docProps._updatedAt);
    }

    if (this._taskpanes) {
      docx = docx.taskpanes();

      for (const e of this.webextensions) {
        let ext = wasm.createWebExtension(
          e._id,
          e._referenceId,
          e._version,
          e._store,
          e._storeType
        );
        for (const [name, value] of Object.entries(e.properties)) {
          ext = ext.property(name, value);
        }
        docx = docx.web_extension(ext);
      }
    }

    for (const item of this.customItems) {
      docx = docx.add_custom_item(item.id, item.xml);
    }

    return docx;
  }

  json() {
    const docx = this.createDocx();
    const json = docx.json_with_update_comments();
    docx.free();
    return JSON.parse(json) as DocxJSON;
  }

  build() {
    const docx = this.createDocx();
    const buf = docx.build(this.hasNumberings);
    // docx.free();
    return buf;
  }
}

export const readDocx = (buf: Uint8Array) => {
  return JSON.parse(wasm.readDocx(buf)) as DocxJSON;
};

export * from "./paragraph";
export * from "./paragraph-property";
export * from "./insert";
export * from "./delete";
export * from "./border";
export * from "./table";
export * from "./table-cell";
export * from "./table-cell-border";
export * from "./table-cell-borders";
export * from "./table-of-contents";
export * from "./table-of-contents-item";
export * from "./table-row";
export * from "./run";
export * from "./text";
export * from "./style";
export * from "./styles";
export * from "./hyperlink";
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
export * from "./webextension";
export * from "./header";
export * from "./footer";
export * from "./image";
