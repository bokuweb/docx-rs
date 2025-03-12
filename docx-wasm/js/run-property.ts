import * as wasm from "./pkg/docx_wasm";

import { BorderType } from "./border";
import { Shading } from "./shading";

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

export class RunProperty {
  _style?: string;
  _size?: number;
  _color?: string;
  _highlight?: string;
  _vertAlign?: VertAlignType;
  _bold?: boolean;
  _italic?: boolean;
  _strike?: boolean;
  _dstrike?: boolean;
  _underline?: string;
  _vanish?: boolean;
  _fonts?: RunFonts;
  _characterSpacing?: number;
  _textBorder?: TextBorder;
  _ins?: RunPropertyIns;
  _del?: RunPropertyDel;
  _shading?: Shading;

  style(style: string) {
    this._style = style;
    return this;
  }

  size(size: number) {
    this._size = size;
    return this;
  }

  color(color: string) {
    this._color = color;
    return this;
  }

  highlight(color: string) {
    this._highlight = color;
    return this;
  }

  vertAlign(vertAlign: VertAlignType) {
    this._vertAlign = vertAlign;
    return this;
  }

  bold() {
    this._bold = true;
    return this;
  }

  disableBold() {
    this._bold = false;
    return this;
  }

  strike() {
    this._strike = true;
    return this;
  }

  disableStrike() {
    this._strike = false;
    return this;
  }

  dstrike() {
    this._dstrike = true;
    return this;
  }

  disableDstrike() {
    this._dstrike = false;
    return this;
  }

  italic() {
    this._italic = true;
    return this;
  }

  disableItalic() {
    this._italic = false;
    return this;
  }

  underline(type: string) {
    this._underline = type;
    return this;
  }

  vanish() {
    this._vanish = true;
    return this;
  }

  fonts(fonts: RunFonts) {
    this._fonts = fonts;
    return this;
  }

  spacing(characterSpacing: number) {
    this._characterSpacing = characterSpacing;
    return this;
  }

  delete(author: string, date: string) {
    this._del = { author, date };
    return this;
  }

  insert(author: string, date: string) {
    this._ins = { author, date };
    return this;
  }

  textBorder(type: BorderType, size: number, space: number, color: string) {
    this._textBorder = {
      borderType: type,
      size,
      space,
      color,
    };
    return this;
  }

  shading(type: string, color: string, fill: string) {
    const s = new Shading();
    s.color(color);
    s.fill(fill);
    s.type(type);
    this._shading = s;
    return this;
  }
}

export const convertBorderType = (t: BorderType) => {
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

export const createDefaultRunProperty = (): RunProperty => {
  return new RunProperty();
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

// @deprecated
export const setRunProperty = <T extends wasm.Run | wasm.Style>(
  target: T,
  property: RunProperty
): T => {
  if (property._style && target instanceof wasm.Run) {
    target = target.style(property._style) as T;
  }

  if (typeof property._size !== "undefined") {
    target = target.size(property._size) as T;
  }

  if (property._color) {
    target = target.color(property._color) as T;
  }

  if (property._highlight) {
    target = target.highlight(property._highlight) as T;
  }

  if (property._vertAlign) {
    if (property._vertAlign === "superscript") {
      target = target.vert_align(wasm.VertAlignType.SuperScript) as T;
    } else if (property._vertAlign === "subscript") {
      target = target.vert_align(wasm.VertAlignType.SubScript) as T;
    }
  }

  if (property._bold) {
    target = target.bold() as T;
  }

  if (property._italic) {
    target = target.italic() as T;
  }

  if (property._strike) {
    target = target.strike() as T;
  }

  if (property._dstrike) {
    target = target.dstrike() as T;
  }

  if (property._underline) {
    target = target.underline(property._underline) as T;
  }

  if (property._vanish) {
    target = target.vanish() as T;
  }

  if (property._characterSpacing != null) {
    target = target.character_spacing(property._characterSpacing) as T;
  }

  if (property._textBorder) {
    const { borderType, color, space, size } = property._textBorder;
    target = target.text_border(
      convertBorderType(borderType),
      size,
      space,
      color
    ) as T;
  }

  if (property._fonts) {
    const fonts = property._fonts.buildWasmObject();
    target = target.fonts(fonts) as T;
  }

  if (property._shading != null) {
    target = target.shading(
      property._shading._type,
      property._shading._color,
      property._shading._fill
    ) as T;
  }

  return target;
};

// @deprecated
export const createRunProperty = (property: RunProperty): wasm.RunProperty => {
  let target = wasm.createRunProperty();
  if (property._style) {
    target = target.style(property._style);
  }

  if (typeof property._size !== "undefined") {
    target = target.size(property._size);
  }

  if (property._color) {
    target = target.color(property._color);
  }

  if (property._highlight) {
    target = target.highlight(property._highlight);
  }

  if (property._vertAlign) {
    if (property._vertAlign === "superscript") {
      target = target.vert_align(wasm.VertAlignType.SuperScript);
    } else if (property._vertAlign === "subscript") {
      target = target.vert_align(wasm.VertAlignType.SubScript);
    }
  }

  if (property._bold != null) {
    if (property._bold) {
      target = target.bold();
    } else {
      target = target.disable_bold();
    }
  }

  if (property._italic != null) {
    if (property._italic) {
      target = target.italic();
    } else {
      target = target.disable_italic();
    }
  }

  if (property._strike != null) {
    if (property._strike) {
      target = target.strike();
    } else {
      target = target.disable_strike();
    }
  }

  if (property._dstrike != null) {
    if (property._dstrike) {
      target = target.dstrike();
    } else {
      target = target.disable_dstrike();
    }
  }

  if (property._underline) {
    target = target.underline(property._underline);
  }

  if (property._vanish) {
    target = target.vanish();
  }

  if (property._characterSpacing != null) {
    target = target.character_spacing(property._characterSpacing);
  }

  if (property._textBorder) {
    const { borderType, color, space, size } = property._textBorder;
    target = target.text_border(
      convertBorderType(borderType),
      size,
      space,
      color
    );
  }

  if (property._fonts) {
    const fonts = property._fonts.buildWasmObject();
    target = target.fonts(fonts);
  }

  if (property._shading != null) {
    target = target.shading(
      property._shading._type,
      property._shading._color,
      property._shading._fill
    );
  }

  return target;
};
