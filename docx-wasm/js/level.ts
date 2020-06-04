import { ParagraphProperty, SpecialIndentKind } from "./paragraph";

export type LevelSuffixType = "nothing" | "tab" | "space";

export class Level {
  id: number;
  start: number;
  format: string;
  text: string;
  jc: string;
  paragraphProperty: ParagraphProperty = {};
  levelSuffix: LevelSuffixType;

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
    this.levelSuffix = "tab";
  }

  indent(
    left: number,
    specialIndentKind?: SpecialIndentKind,
    specialIndentSize?: number
  ) {
    this.paragraphProperty.indent = {
      left,
      specialIndentKind,
      specialIndentSize,
    };
    return this;
  }

  suffix(s: LevelSuffixType) {
    this.levelSuffix = s;
    return this;
  }
}

export class LevelOverride {
  level: number;
  startOverride: number | null = null;

  constructor(level: number) {
    this.level = level;
  }

  start(start: number) {
    this.startOverride = start;
    return this;
  }
}
