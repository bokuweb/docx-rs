import * as wasm from "./pkg";

import { createDefaultTableCellMargins, TableProperty } from "./table";
import { RunProperty, createDefaultRunProperty } from "./run";
import { createDefaultParagraphProperty, ParagraphProperty } from "./paragraph";

export type StyleType =
  | "paragraph"
  | "character"
  | "numbering"
  | "table"
  | "unsupported";

export class Style {
  _styleId: string;
  _name: string;
  _styleType: StyleType;
  _runProperty: RunProperty;
  _paragraphProperty: ParagraphProperty;
  _tableProperty: TableProperty;
  _basedOn: string | null;

  constructor(id: string, type: StyleType) {
    this._styleId = id;
    this._styleType = type;
    this._name = "";
    this._runProperty = {};
    this._tableProperty = {
      cellMargins: createDefaultTableCellMargins(),
    };
    this._runProperty = createDefaultRunProperty();
    this._paragraphProperty = createDefaultParagraphProperty();
    this._basedOn = null;
  }

  name = (n: string) => {
    this._name = n;
    return this;
  };

  basedOn = (n: string) => {
    this._basedOn = n;
    return this;
  };

  // TODO:
  // runProperty = (n: RunProperty) => {
  //   this._runProperty = n;
  //   return this;
  // };

  // TODO:
  // paragraphProperty = (n: ParagraphProperty) => {
  //   this._paragraphProperty = n;
  //   return this;
  // };

  tableProperty = (n: TableProperty) => {
    this._tableProperty = n;
    return this;
  };

  buildStyleType = () => {
    switch (this._styleType) {
      case "character":
        return wasm.StyleType.Character;
      case "numbering":
        return wasm.StyleType.Numbering;
      case "paragraph":
        return wasm.StyleType.Paragraph;
      case "table":
        return wasm.StyleType.Table;
    }
    return wasm.StyleType.Paragraph;
  };

  buildWasmObject = () => {
    const styleType = this.buildStyleType();
    let s = wasm.createStyle(this._styleId, styleType);

    if (this._name) {
      s = s.name(this._name);
    }

    if (this._basedOn) {
      s = s.based_on(this._basedOn);
    }

    return s;
  };
}
