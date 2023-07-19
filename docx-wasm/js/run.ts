import * as wasm from "./pkg";

import { Text } from "./text";
import { DeleteText } from "./delete-text";
import { Tab } from "./tab";
import { Break, BreakType } from "./break";
import { BorderType } from "./border";
import { Image } from "./image";

export type RunChild = Text | DeleteText | Tab | Break | Image;

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
  characterSpacing?: number;
  textBorder?: TextBorder;
  ins?: RunPropertyIns;
  del?: RunPropertyDel;
};

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

  addImage(image: Image) {
    this.children.push(image);
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

  spacing(characterSpacing: number) {
    this.property = { ...this.property, characterSpacing };
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

  build() {
    let run = wasm.createRun();
    this.children.forEach((child) => {
      if (child instanceof Text) {
        run = run.add_text(child.text);
      } else if (child instanceof DeleteText) {
        run = run.add_delete_text(child.text);
      } else if (child instanceof Tab) {
        run = run.add_tab();
      } else if (child instanceof Break) {
        if (child.type === "column") {
          run = run.add_break(wasm.BreakType.Column);
        } else if (child.type === "page") {
          run = run.add_break(wasm.BreakType.Page);
        } else if (child.type === "textWrapping") {
          run = run.add_break(wasm.BreakType.TextWrapping);
        }
      } else if (child instanceof Image) {
        let pic = wasm.createPic(child.data);
        if (child.w != null && child.h != null) {
          pic = pic.size(child.w, child.h);
        }
        if (child._floating) {
          pic = pic.floating();
        }
        if (child._offsetX != null) {
          pic = pic.offset_x(child._offsetX);
        }
        if (child._offsetY != null) {
          pic = pic.offset_x(child._offsetY);
        }
        if (child.rot != null) {
          pic = pic.rotate(child.rot);
        }
        run = run.add_image(pic);
      }
    });

    run = setRunProperty(run, this.property) as wasm.Run;

    return run;
  }
}

export const setRunProperty = <T extends wasm.Run | wasm.Style>(
  target: T,
  property: RunProperty
): T => {
  if (property.style && target instanceof wasm.Run) {
    target = target.style(property.style) as T;
  }

  if (typeof property.size !== "undefined") {
    target = target.size(property.size) as T;
  }

  if (property.color) {
    target = target.color(property.color) as T;
  }

  if (property.highlight) {
    target = target.highlight(property.highlight) as T;
  }

  if (property.vertAlign) {
    if (property.vertAlign === "superscript") {
      target = target.vert_align(wasm.VertAlignType.SuperScript) as T;
    } else if (property.vertAlign === "subscript") {
      target = target.vert_align(wasm.VertAlignType.SubScript) as T;
    }
  }

  if (property.bold) {
    target = target.bold() as T;
  }

  if (property.italic) {
    target = target.italic() as T;
  }

  if (property.strike) {
    target = target.strike() as T;
  }

  if (property.underline) {
    target = target.underline(property.underline) as T;
  }

  if (property.vanish) {
    target = target.vanish() as T;
  }

  if (property.characterSpacing != null) {
    target = target.character_spacing(property.characterSpacing) as T;
  }

  if (property.textBorder) {
    const { borderType, color, space, size } = property.textBorder;
    target = target.text_border(
      convertBorderType(borderType),
      size,
      space,
      color
    ) as T;
  }

  if (property.fonts) {
    const fonts = property.fonts.buildWasmObject();
    target = target.fonts(fonts) as T;
  }

  return target;
};
