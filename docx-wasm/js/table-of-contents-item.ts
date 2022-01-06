import * as wasm from "./pkg/docx_wasm";

export class TableOfContentsItem {
  _text = "";
  _tocKey = "";
  _level = 1;
  _pageRef = "";

  text = (text: string) => {
    this._text = text;
    return this;
  };

  tocKey = (key: string) => {
    this._tocKey = key;
    return this;
  };

  level = (l: number) => {
    this._level = l;
    return this;
  };

  pageRef = (r: string) => {
    this._pageRef = r;
    return this;
  };

  buildWasmObject = () => {
    let item = wasm.createTableOfContentsItem();
    if (this._text) {
      item = item.text(this._text);
    }

    if (this._tocKey) {
      item = item.toc_key(this._tocKey);
    }

    if (this._level) {
      item = item.level(this._level);
    }

    if (this._pageRef) {
      item = item.page_ref(this._pageRef);
    }

    return item;
  };
}
