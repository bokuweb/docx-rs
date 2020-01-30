import { ParagraphProperty, SpecialIndentKind } from "./paragraph";

export class Level {
  id: number;
  start: number;
  format: string;
  text: string;
  jc: string;
  paragraphProperty: ParagraphProperty = {};

  constructor(
    id: number,
    start: number,
    format: string,
    text: string,
    jc: string
  ) {
    this.id = id;
    this.start = start;
    this.format = format;
    this.text = text;
    this.jc = jc;
  }

  indent(
    left: number,
    specialIndentKind?: SpecialIndentKind,
    specialIndentSize?: number
  ) {
    this.paragraphProperty.indent = {
      left,
      specialIndentKind,
      specialIndentSize
    };
    return this;
  }
}
