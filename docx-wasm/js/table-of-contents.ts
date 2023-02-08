import { Paragraph } from "./paragraph";
import * as wasm from "./pkg";
import { Table } from "./table";
import { TableOfContentsItem } from "./table-of-contents-item";
import { build } from "./builder";

export class TableOfContents {
  _instrText?: string;
  _headingStylesRange: [number, number] | null = null;
  _styleWithLevels: { styleId: string; level: number }[] = [];
  _hyperlink = false;
  _alias = "";
  _auto = false;
  _withoutSdt = false;
  _dirty = false;
  _items: TableOfContentsItem[] = [];
  _pageRefPlaceholder = "";
  _beforeContents: (Paragraph | Table)[] = [];
  _afterContents: (Paragraph | Table)[] = [];
  _delete: { author: string; date: string } | null = null;

  constructor(instrText?: string) {
    this._instrText = instrText;
  }

  addBeforeParagraph(p: Paragraph) {
    this._beforeContents.push(p);
    return this;
  }

  addBeforeTable(t: Table) {
    this._beforeContents.push(t);
    return this;
  }

  addAfterParagraph(p: Paragraph) {
    this._afterContents.push(p);
    return this;
  }

  addAfterTable(t: Table) {
    this._afterContents.push(t);
    return this;
  }

  headingStylesRange = (r: [number, number]) => {
    this._headingStylesRange = r;
    return this;
  };

  addStyleWithLevel = (styleId: string, level: number) => {
    this._styleWithLevels.push({ styleId, level });
    return this;
  };

  hyperlink = () => {
    this._hyperlink = true;
    return this;
  };

  alias = (alias: string) => {
    this._alias = alias;
    return this;
  };

  pageRefPlaceholder = (placeholder: string) => {
    this._pageRefPlaceholder = placeholder;
    return this;
  };

  auto = () => {
    this._auto = true;
    return this;
  };

  dirty = () => {
    this._dirty = true;
    return this;
  };

  withoutSdt = () => {
    this._withoutSdt = true;
    return this;
  };

  delete = (author: string, date: string) => {
    this._delete = { author, date };
    return this;
  };

  addItem = (item: TableOfContentsItem) => {
    this._items.push(item);
    return this;
  };

  buildWasmObject = () => {
    let toc = this._instrText
      ? wasm.createTableOfContentsWithInstrText(this._instrText)
      : wasm.createTableOfContents();
    if (this._headingStylesRange) {
      toc = toc.heading_styles_range(
        this._headingStylesRange[0],
        this._headingStylesRange[1]
      );
    }

    if (this._hyperlink) {
      toc = toc.hyperlink();
    }

    if (this._alias) {
      toc = toc.alias(this._alias);
    }

    if (this._auto) {
      toc = toc.auto();
    }

    if (this._dirty) {
      toc = toc.dirty();
    }

    if (this._withoutSdt) {
      toc = toc.without_sdt();
    }

    if (this._pageRefPlaceholder) {
      toc = toc.page_ref_placeholder(this._pageRefPlaceholder);
    }

    if (this._delete) {
      toc = toc.delete(this._delete.author, this._delete.date);
    }

    for (const sl of this._styleWithLevels) {
      toc = toc.add_style_with_level(sl.styleId, sl.level);
    }

    for (const item of this._items) {
      toc = toc.add_item(item.buildWasmObject());
    }

    for (const c of this._beforeContents) {
      if (c instanceof Paragraph) {
        toc = toc.add_before_paragraph(build(c));
      } else if (c instanceof Table) {
        toc = toc.add_before_table(c.build());
      }
    }

    for (const c of this._afterContents) {
      if (c instanceof Paragraph) {
        toc = toc.add_after_paragraph(build(c));
      } else if (c instanceof Table) {
        toc = toc.add_after_table(c.build());
      }
    }

    return toc;
  };
}
