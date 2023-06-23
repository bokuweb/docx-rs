import * as wasm from "./pkg";

import {
  createDefaultTableCellMargins,
  setTableProperty,
  TableAlignmentType,
  TableLayoutType,
  TableProperty,
} from "./table";
import {
  RunProperty,
  createDefaultRunProperty,
  VertAlignType,
  RunFonts,
  setRunProperty,
} from "./run";
import {
  AlignmentType,
  createDefaultParagraphProperty,
  LineSpacing,
  ParagraphProperty,
  setParagraphProperty,
  SpecialIndentKind,
} from "./paragraph-property";
import { BorderType } from "./border";
import { WidthType } from ".";

export type StyleType = "paragraph" | "character" | "numbering" | "table";

export class Style {
  _styleId: string;
  _name: string;
  _styleType: StyleType;
  _runProperty: RunProperty;
  _paragraphProperty: ParagraphProperty;
  _tableProperty: TableProperty;
  _basedOn: string | null;
  _link: string | null;

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
    this._link = null;
  }

  name = (n: string) => {
    this._name = n;
    return this;
  };

  basedOn = (n: string) => {
    this._basedOn = n;
    return this;
  };

  link = (n: string) => {
    this._link = n;
    return this;
  };

  // TODO:
  // runProperty = (n: RunProperty) => {
  //   this._runProperty = n;
  //   return this;
  // };

  // run property
  style(style: string) {
    this._runProperty = { ...this._runProperty, style };
    return this;
  }

  size(size: number) {
    this._runProperty = { ...this._runProperty, size };
    return this;
  }

  color(color: string) {
    this._runProperty = { ...this._runProperty, color };
    return this;
  }

  highlight(color: string) {
    this._runProperty = { ...this._runProperty, highlight: color };
    return this;
  }

  vertAlign(vertAlign: VertAlignType) {
    this._runProperty = { ...this._runProperty, vertAlign };
    return this;
  }

  bold() {
    this._runProperty = { ...this._runProperty, bold: true };
    return this;
  }

  strike() {
    this._runProperty = { ...this._runProperty, strike: true };
    return this;
  }

  italic() {
    this._runProperty = { ...this._runProperty, italic: true };
    return this;
  }

  underline(type: string) {
    this._runProperty = { ...this._runProperty, underline: type };
    return this;
  }

  vanish() {
    this._runProperty = { ...this._runProperty, vanish: true };
    return this;
  }

  fonts(fonts: RunFonts) {
    this._runProperty = { ...this._runProperty, fonts };
    return this;
  }

  characterSpacing(characterSpacing: number) {
    this._runProperty = { ...this._runProperty, characterSpacing };
    return this;
  }

  delete(author: string, date: string) {
    this._runProperty = { ...this._runProperty, del: { author, date } };
    return this;
  }

  insert(author: string, date: string) {
    this._runProperty = { ...this._runProperty, ins: { author, date } };
    return this;
  }

  textBorder(type: BorderType, size: number, space: number, color: string) {
    this._runProperty = {
      ...this._runProperty,
      textBorder: {
        borderType: type,
        size,
        space,
        color,
      },
    };
    return this;
  }

  // TODO:
  // paragraphProperty = (n: ParagraphProperty) => {
  //   this._paragraphProperty = n;
  //   return this;
  // };

  // paragraph property
  align(type: AlignmentType) {
    this._paragraphProperty.align = type;
    return this;
  }

  indent(
    left: number,
    specialIndentKind?: SpecialIndentKind,
    specialIndentSize?: number
  ) {
    this._paragraphProperty.indent = {
      left,
      specialIndentKind,
      specialIndentSize,
    };
    return this;
  }

  numbering(id: number, level: number) {
    this._paragraphProperty.numbering = { id, level };
    return this;
  }

  lineSpacing(spacing: LineSpacing) {
    this._paragraphProperty.lineSpacing = spacing;
    return this;
  }

  keepNext(v: boolean) {
    this._paragraphProperty = { ...this._paragraphProperty, keepNext: v };
    return this;
  }

  keepLines(v: boolean) {
    this._paragraphProperty = { ...this._paragraphProperty, keepLines: v };
    return this;
  }

  pageBreakBefore(v: boolean) {
    this._paragraphProperty = {
      ...this._paragraphProperty,
      pageBreakBefore: v,
    };
    return this;
  }

  widowControl(v: boolean) {
    this._paragraphProperty = { ...this._paragraphProperty, widowControl: v };
    return this;
  }

  outlineLevel(v: number) {
    this._paragraphProperty = { ...this._paragraphProperty, outlineLvl: v };
    return this;
  }

  // tableProperty = (n: TableProperty) => {
  //   this._tableProperty = n;
  //   return this;
  // };

  // tableProperty
  tableIndent(v: number) {
    this._tableProperty.indent = v;
    return this;
  }

  tableAlign(v: TableAlignmentType) {
    this._tableProperty.align = v;
    return this;
  }

  layout(l: TableLayoutType) {
    this._tableProperty.layout = l;
    return this;
  }

  width(w: number) {
    this._tableProperty.width = w;
    return this;
  }

  cellMargins(top: number, right: number, bottom: number, left: number) {
    this._tableProperty.cellMargins = {
      top: { val: top, type: "dxa" },
      left: { val: left, type: "dxa" },
      bottom: { val: bottom, type: "dxa" },
      right: { val: right, type: "dxa" },
    };
    return this;
  }

  cellMarginTop(v: number, t: WidthType) {
    this._tableProperty.cellMargins.top = { val: v, type: t };
    return this;
  }

  cellMarginLeft(v: number, t: WidthType) {
    this._tableProperty.cellMargins.left = { val: v, type: t };
    return this;
  }

  cellMarginRight(v: number, t: WidthType) {
    this._tableProperty.cellMargins.right = { val: v, type: t };
    return this;
  }

  cellMarginBottom(v: number, t: WidthType) {
    this._tableProperty.cellMargins.bottom = { val: v, type: t };
    return this;
  }

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

    if (this._link) {
      s = s.link(this._link);
    }

    s = setRunProperty(s, this._runProperty);

    s = setParagraphProperty(s, this._paragraphProperty);

    s = setTableProperty(s, this._tableProperty);

    return s;
  };
}
