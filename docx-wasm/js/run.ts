import * as wasm from "./pkg";

import { Text } from "./text";
import { DeleteText } from "./delete-text";
import { Tab } from "./tab";
import { Break, BreakType } from "./break";
import { BorderType } from "./border";

export type RunChild = Text | DeleteText | Tab | Break;

export type TextBorder = {
  borderType: BorderType;
  color: string;
  space: number;
  size: number;
};

export type VertAlignType = "baseline" | "superscript" | "subscript";

export type RunPropertyDel = {
  author: string;
  date: string;
};

export type RunPropertyIns = {
  author: string;
  date: string;
};

export type RunProperty = {
  style?: string;
  size?: number;
  color?: string;
  highlight?: string;
  vertAlign?: VertAlignType;
  bold?: boolean;
  italic?: boolean;
  strike?: boolean;
  underline?: string;
  vanish?: boolean;
  fonts?: RunFonts;
  spacing?: number;
  textBorder?: TextBorder;
  ins?: RunPropertyIns;
  del?: RunPropertyDel;
};

export const createDefaultRunProperty = (): RunProperty => {
  return {};
};

export class RunFonts {
  _ascii?: string;
  _hiAnsi?: string;
  _eastAsia?: string;
  _cs?: string;
  _asciiTheme?: string;
  _hiAnsiTheme?: string;
  _eastAsiaTheme?: string;
  _csTheme?: string;
  _hint?: string;

  ascii(f: string) {
    this._ascii = f;
    return this;
  }

  hiAnsi(f: string) {
    this._hiAnsi = f;
    return this;
  }

  cs(f: string) {
    this._cs = f;
    return this;
  }

  eastAsia(f: string) {
    this._eastAsia = f;
    return this;
  }

  asciiTheme(f: string) {
    this._asciiTheme = f;
    return this;
  }

  hiAnsiTheme(f: string) {
    this._hiAnsiTheme = f;
    return this;
  }

  csTheme(f: string) {
    this._csTheme = f;
    return this;
  }

  eastAsiaTheme(f: string) {
    this._eastAsia = f;
    return this;
  }

  hint(f: string) {
    this._hint = f;
    return this;
  }

  buildWasmObject = () => {
    let f = wasm.createRunFonts();
    if (this?._ascii) {
      f = f.ascii(this._ascii);
    }
    if (this?._hiAnsi) {
      f = f.hi_ansi(this._hiAnsi);
    }
    if (this?._cs) {
      f = f.cs(this._cs);
    }
    if (this?._eastAsia) {
      f = f.east_asia(this._eastAsia);
    }

    // theme
    if (this?._asciiTheme) {
      f = f.ascii_theme(this._asciiTheme);
    }
    if (this?._hiAnsiTheme) {
      f = f.hi_ansi_theme(this._hiAnsiTheme);
    }
    if (this?._csTheme) {
      f = f.cs_theme(this._csTheme);
    }
    if (this?._eastAsiaTheme) {
      f = f.east_asia_theme(this._eastAsiaTheme);
    }

    if (this?._hint) {
      f = f.hint(this._hint);
    }
    return f;
  };
}

export class Run {
  children: RunChild[] = [];
  property: RunProperty = {};

  addText(text: string) {
    this.children.push(new Text(text));
    return this;
  }

  addDeleteText(text: string) {
    this.children.push(new DeleteText(text));
    return this;
  }

  addTab() {
    this.children.push(new Tab());
    return this;
  }

  addBreak(type: BreakType) {
    this.children.push(new Break(type));
    return this;
  }

  style(style: string) {
    this.property = { ...this.property, style };
    return this;
  }

  size(size: number) {
    this.property = { ...this.property, size };
    return this;
  }

  color(color: string) {
    this.property = { ...this.property, color };
    return this;
  }

  highlight(color: string) {
    this.property = { ...this.property, highlight: color };
    return this;
  }

  vertAlign(vertAlign: VertAlignType) {
    this.property = { ...this.property, vertAlign };
    return this;
  }

  bold() {
    this.property = { ...this.property, bold: true };
    return this;
  }

  strike() {
    this.property = { ...this.property, strike: true };
    return this;
  }

  italic() {
    this.property = { ...this.property, italic: true };
    return this;
  }

  underline(type: string) {
    this.property = { ...this.property, underline: type };
    return this;
  }

  vanish() {
    this.property = { ...this.property, vanish: true };
    return this;
  }

  fonts(fonts: RunFonts) {
    this.property = { ...this.property, fonts };
    return this;
  }

  spacing(spacing: number) {
    this.property = { ...this.property, spacing };
    return this;
  }

  delete(author: string, date: string) {
    this.property = { ...this.property, del: { author, date } };
    return this;
  }

  insert(author: string, date: string) {
    this.property = { ...this.property, ins: { author, date } };
    return this;
  }

  textBorder(type: BorderType, size: number, space: number, color: string) {
    this.property = {
      ...this.property,
      textBorder: {
        borderType: type,
        size,
        space,
        color,
      },
    };
    return this;
  }
}
