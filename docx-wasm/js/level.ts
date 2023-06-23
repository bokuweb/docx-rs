import { LevelJSON } from "./json";
import {
  createDefaultParagraphProperty,
  ParagraphProperty,
  SpecialIndentKind,
} from "./paragraph-property";
import { RunFonts, RunProperty } from "./run";

export type LevelSuffixType = "nothing" | "tab" | "space";

export class Level {
  id: number;
  start: number;
  format: string;
  text: string;
  jc: string;
  paragraphProperty: ParagraphProperty = createDefaultParagraphProperty();
  runProperty: RunProperty = {};
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

  size(size: number) {
    this.runProperty = { ...this.runProperty, size };
    return this;
  }

  color(color: string) {
    this.runProperty = { ...this.runProperty, color };
    return this;
  }

  highlight(color: string) {
    this.runProperty = { ...this.runProperty, highlight: color };
    return this;
  }

  bold() {
    this.runProperty = { ...this.runProperty, bold: true };
    return this;
  }

  italic() {
    this.runProperty = { ...this.runProperty, italic: true };
    return this;
  }

  underline(type: string) {
    this.runProperty = { ...this.runProperty, underline: type };
    return this;
  }

  vanish() {
    this.runProperty = { ...this.runProperty, vanish: true };
    return this;
  }

  fonts(fonts: RunFonts) {
    this.runProperty = { ...this.runProperty, fonts };
    return this;
  }

  characterSpacing(characterSpacing: number) {
    this.runProperty = { ...this.runProperty, characterSpacing };
    return this;
  }
}

export class LevelOverride {
  level: number;
  startOverride: number | null = null;
  levelOverride: LevelJSON | null = null;

  constructor(level: number) {
    this.level = level;
  }

  overrideStart(start: number) {
    this.startOverride = start;
    return this;
  }

  overrideLevel(level: LevelJSON) {
    this.levelOverride = level;
    return this;
  }
}
