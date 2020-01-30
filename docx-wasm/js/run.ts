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
};

export class Run {
  children: RunChild[] = [];
  property: RunProperty;

  addText(text: Text) {
    this.children.push(text);
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
    this.property.size = size;
    return this;
  }

  color(color: string) {
    this.property.color = color;
    return this;
  }

  highlight(color: string) {
    this.property.highlight = color;
    return this;
  }

  bold() {
    this.property.bold = true;
    return this;
  }

  italic() {
    this.property.italic = true;
    return this;
  }

  underline(type: string) {
    this.property.underline = type;
    return this;
  }

  vanish() {
    this.property.vanish = true;
    return this;
  }
}
