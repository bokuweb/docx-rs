import { RunProperty, createDefaultRunProperty } from "./run";

import * as wasm from "./pkg";

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

export const createParagraphAlignment = (
  align?: AlignmentType | undefined
): wasm.AlignmentType | null => {
  switch (align) {
    case "center": {
      return wasm.AlignmentType.Center;
    }
    case "right": {
      return wasm.AlignmentType.Right;
    }
    case "justified": {
      return wasm.AlignmentType.Justified;
    }
    case "left": {
      return wasm.AlignmentType.Left;
    }
    case "distribute": {
      return wasm.AlignmentType.Distribute;
    }
    case "both": {
      return wasm.AlignmentType.Both;
    }
    case "end": {
      return wasm.AlignmentType.End;
    }
    default: {
      return null;
    }
  }
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

  align(type: AlignmentType) {
    this._property.align = type;
    return this;
  }

  style(id: string) {
    this._property.styleId = id;
    return this;
  }

  indent(
    left: number,
    specialIndentKind?: SpecialIndentKind,
    specialIndentSize?: number
  ) {
    this._property.indent = { left, specialIndentKind, specialIndentSize };
    return this;
  }

  numbering(id: number, level: number) {
    this._property.numbering = { id, level };
    return this;
  }
}

export const buildLineSpacing = (
  p: ParagraphProperty
): wasm.LineSpacing | null => {
  const { lineSpacing } = p;
  if (lineSpacing == null) return null;
  let kind;
  switch (lineSpacing._lineRule) {
    case "atLeast": {
      kind = wasm.LineSpacingType.AtLeast;
      break;
    }
    case "auto": {
      kind = wasm.LineSpacingType.Auto;
      break;
    }
    case "exact": {
      kind = wasm.LineSpacingType.Exact;
      break;
    }
  }
  let spacing = wasm.createLineSpacing();
  if (lineSpacing._before != null) {
    spacing = spacing.before(lineSpacing._before);
  }

  if (lineSpacing._after != null) {
    spacing = spacing.after(lineSpacing._after);
  }

  if (lineSpacing._beforeLines != null) {
    spacing = spacing.before_lines(lineSpacing._beforeLines);
  }

  if (lineSpacing._afterLines != null) {
    spacing = spacing.after_lines(lineSpacing._afterLines);
  }

  if (lineSpacing._line != null) {
    spacing = spacing.line(lineSpacing._line);
  }

  if (kind != null) {
    spacing = spacing.line_rule(kind);
  }
  return spacing;
};

export const setParagraphProperty = <T extends wasm.Paragraph | wasm.Style>(
  target: T,
  property: ParagraphProperty
): T => {
  const alignment = createParagraphAlignment(property.align);
  if (alignment != null) {
    target = target.align(alignment) as T;
  }

  if (typeof property.indent !== "undefined") {
    const { indent } = property;
    let kind;
    switch (property.indent.specialIndentKind) {
      case "firstLine": {
        kind = wasm.SpecialIndentKind.FirstLine;
        break;
      }
      case "hanging": {
        kind = wasm.SpecialIndentKind.Hanging;
        break;
      }
    }
    target = target.indent(indent.left, kind, indent.specialIndentSize) as T;
  }

  if (typeof property.numbering !== "undefined") {
    const { numbering } = property;
    target = target.numbering(numbering.id, numbering.level) as T;
  }

  if (property.runProperty.bold) {
    target = target.bold() as T;
  }

  if (typeof property.lineSpacing !== "undefined") {
    const spacing = buildLineSpacing(property);
    if (spacing) {
      target = target.line_spacing(spacing) as T;
    }
  }

  if (property.runProperty.italic) {
    target = target.italic() as T;
  }

  if (property.runProperty.size) {
    target = target.size(property.runProperty.size) as T;
  }

  if (property.runProperty.fonts) {
    let f = wasm.createRunFonts();
    if (property.runProperty.fonts._ascii) {
      f = f.ascii(property.runProperty.fonts._ascii);
    }
    if (property.runProperty.fonts._hiAnsi) {
      f = f.hi_ansi(property.runProperty.fonts._hiAnsi);
    }
    if (property.runProperty.fonts._cs) {
      f = f.cs(property.runProperty.fonts._cs);
    }
    if (property.runProperty.fonts._eastAsia) {
      f = f.east_asia(property.runProperty.fonts._eastAsia);
    }
    target = target.fonts(f) as T;
  }

  if (property.keepLines) {
    target = target.keep_lines(true) as T;
  }

  if (property.keepNext) {
    target = target.keep_next(true) as T;
  }

  if (property.pageBreakBefore) {
    target = target.page_break_before(true) as T;
  }

  if (property.widowControl) {
    target = target.widow_control(true) as T;
  }

  return target;
};
