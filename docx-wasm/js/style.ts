import { RunProperty, createDefaultRunProperty } from "./run";
import { createDefaultParagraphProperty, ParagraphProperty } from "./paragraph";
import { TableProperty } from "./table";

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
    this._runProperty = createDefaultRunProperty();
    this._paragraphProperty = createDefaultParagraphProperty();
    this._tableProperty = {};
    this._basedOn = null;
  }

  // TODO: Add setter
}
