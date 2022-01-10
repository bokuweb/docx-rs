import * as wasm from "./pkg";

import { TableOfContentsItem } from "./table-of-contents-item";

export class TableOfContents {
  _headingStylesRange: [number, number] | null = null;
  _styleWithLevels: { styleId: string; level: number }[] = [];
  _hyperlink = false;
  _alias = "";
  _auto = false;
  _dirty = false;
  _items: TableOfContentsItem[] = [];
  _pageRefPlaceholder = "";

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

  addItem = (item: TableOfContentsItem) => {
    this._items.push(item);
    return this;
  };

  buildWasmObject = () => {
    let toc = wasm.createTableOfContents();
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

    if (this._pageRefPlaceholder) {
      toc = toc.page_ref_placeholder(this._pageRefPlaceholder);
    }

    for (const sl of this._styleWithLevels) {
      toc = toc.add_style_with_level(sl.styleId, sl.level);
    }

    for (const item of this._items) {
      toc = toc.add_item(item.buildWasmObject());
    }

    return toc;
  };
}
