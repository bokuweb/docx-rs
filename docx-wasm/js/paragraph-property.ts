import { RunProperty, createDefaultRunProperty } from "./run";

export type AlignmentType =
  | "center"
  | "left"
  | "right"
  | "both"
  | "justified"
  | "distribute"
  | "end";

export type SpecialIndentKind = "firstLine" | "hanging";

export type LineSpacingType = "atLeast" | "auto" | "exact";

export class LineSpacing {
  _before?: number;
  _after?: number;
  _beforeLines?: number;
  _afterLines?: number;
  _line?: number;
  _lineRule?: LineSpacingType;

  before(v: number) {
    this._before = v;
    return this;
  }
  after(v: number) {
    this._after = v;
    return this;
  }
  beforeLines(v: number) {
    this._beforeLines = v;
    return this;
  }
  afterLines(v: number) {
    this._afterLines = v;
    return this;
  }
  line(v: number) {
    this._line = v;
    return this;
  }
  lineRule(v: LineSpacingType) {
    this._lineRule = v;
    return this;
  }
}

export type ParagraphProperty = {
  align?: AlignmentType;
  styleId?: string;
  indent?: {
    left: number;
    specialIndentKind?: SpecialIndentKind;
    specialIndentSize?: number;
  };
  numbering?: {
    id: number;
    level: number;
  };
  lineSpacing?: LineSpacing;
  runProperty: RunProperty;
  keepNext: boolean;
  keepLines: boolean;
  pageBreakBefore: boolean;
  widowControl: boolean;
  paragraphPropertyChange?: ParagraphPropertyChange;
};

export const createDefaultParagraphProperty = (): ParagraphProperty => {
  return {
    runProperty: createDefaultRunProperty(),
    keepNext: false,
    keepLines: false,
    pageBreakBefore: false,
    widowControl: false,
  };
};

export class ParagraphPropertyChange {
  _author: string = "";
  _date: string = "";
  _property: ParagraphProperty = createDefaultParagraphProperty();

  author(a: string) {
    this._author = a;
    return this;
  }

  date(a: string) {
    this._date = a;
    return this;
  }

  property(p: ParagraphProperty) {
    this._property = p;
    return this;
  }
}
