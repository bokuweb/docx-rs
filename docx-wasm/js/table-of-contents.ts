import * as wasm from "./pkg";

export class TableOfContents {
  _headingStylesRange: [number, number] | null = null;
  _hyperlink = false;
  _alias = "";
  _disableAutoItems = false;
  _dirty = false;

  headingStylesRange = (r: [number, number]) => {
    this._headingStylesRange = r;
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

  disableAutoItems = () => {
    this._disableAutoItems = true;
    return this;
  };

  dirty = () => {
    this._dirty = true;
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

    if (this._disableAutoItems) {
      toc = toc.disable_auto_items();
    }

    if (this._dirty) {
      toc = toc.dirty();
    }

    return toc;
  };
}
