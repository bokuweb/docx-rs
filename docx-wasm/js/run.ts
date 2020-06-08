import { Text } from "./text";
import { DeleteText } from "./delete-text";
import { Tab } from "./tab";
import { Break, BreakType } from "./break";

export type RunChild = Text | DeleteText | Tab | Break;

export type RunProperty = {
  size?: number;
  color?: string;
  highlight?: string;
  bold?: boolean;
  italic?: boolean;
  underline?: string;
  vanish?: boolean;
  fonts?: RunFonts;
};

export class RunFonts {
  _ascii?: string;
  _hiAnsi?: string;
  _eastAsia?: string;
  _cs?: string;

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

  bold() {
    this.property = { ...this.property, bold: true };
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
}
