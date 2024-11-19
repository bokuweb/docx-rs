import { RunProperty, createDefaultRunProperty } from "./run";

import * as wasm from "./pkg";
import { TextAlignmentType } from "./json/bindings/TextAlignmentType";
import { Tab } from "./json/bindings/Tab";
import { AlignmentType } from "./json/bindings/AlignmentType";

export { AlignmentType } from "./json/bindings/AlignmentType";

export type SpecialIndentKind = "firstLine" | "hanging";

export type LineSpacingType = "atLeast" | "auto" | "exact";

export type FrameProperty = {
  h?: number;
  hRule?: string;
  hAnchor?: string;
  hSpace?: number;
  vAnchor?: string;
  vSpace?: number;
  w?: number;
  wrap?: string;
  x?: number;
  xAlign?: string;
  y?: number;
  yAlign?: string;
};

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
  textAlignment?: TextAlignmentType;
  styleId?: string;
  indent?: {
    left: number;
    specialIndentKind?: SpecialIndentKind;
    specialIndentSize?: number;
    right?: number;
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
  outlineLvl?: number | null;
  snapToGrid?: boolean;
  adjustRightInd?: number;
  tabs?: Tab[];
  frameProperty?: FrameProperty;
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

export const createParagraphTextAlignment = (
  align?: TextAlignmentType | undefined
): wasm.TextAlignmentType | null => {
  switch (align) {
    case "auto": {
      return wasm.TextAlignmentType.Auto;
    }
    case "baseline": {
      return wasm.TextAlignmentType.Baseline;
    }
    case "bottom": {
      return wasm.TextAlignmentType.Bottom;
    }
    case "center": {
      return wasm.TextAlignmentType.Center;
    }
    case "top": {
      return wasm.TextAlignmentType.Top;
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

  textAlignment(type: TextAlignmentType) {
    this._property.textAlignment = type;
    return this;
  }

  adjustRightInd(v: number) {
    this._property.adjustRightInd = v;
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

// @deprecated
export const setParagraphProperty = <T extends wasm.Paragraph | wasm.Style>(
  target: T,
  property: ParagraphProperty
): T => {
  const alignment = createParagraphAlignment(property.align);
  if (alignment != null) {
    target = target.align(alignment) as T;
  }

  const textAlignment = createParagraphTextAlignment(property.textAlignment);
  if (textAlignment != null) {
    target = target.text_alignment(textAlignment) as T;
  }

  if (property.adjustRightInd != null) {
    target = target.adjust_right_ind(property.adjustRightInd) as T;
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
    target = target.indent(
      indent.left,
      kind,
      indent.specialIndentSize,
      indent.right
    ) as T;
  }

  if (typeof property.numbering !== "undefined") {
    const { numbering } = property;
    target = target.numbering(numbering.id, numbering.level) as T;
  }

  if (property.runProperty.bold) {
    target = target.bold() as T;
  }

  if (property.runProperty.color) {
    target = target.color(property.runProperty.color) as T;
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

  if (property.snapToGrid != null) {
    target = target.snap_to_grid(!!property.snapToGrid) as T;
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

  if (property.outlineLvl != null) {
    target = target.outline_lvl(property.outlineLvl) as T;
  }

  if (property.tabs) {
    for (const tab of property.tabs) {
      let val: wasm.TabValueType | undefined;
      let leader: wasm.TabLeaderType | undefined;
      switch (tab.val) {
        case "bar":
          val = wasm.TabValueType.Bar;
          break;
        case "bar":
          val = wasm.TabValueType.Bar;
          break;
        case "center":
          val = wasm.TabValueType.Center;
          break;
        case "clear":
          val = wasm.TabValueType.Clear;
          break;
        case "decimal":
          val = wasm.TabValueType.Decimal;
          break;
        case "end":
          val = wasm.TabValueType.End;
          break;
        case "right":
          val = wasm.TabValueType.Right;
          break;
        case "num":
          val = wasm.TabValueType.Num;
          break;
        case "start":
          val = wasm.TabValueType.Start;
          break;
        case "left":
          val = wasm.TabValueType.Left;
          break;
      }

      switch (tab.leader) {
        case "dot":
          leader = wasm.TabLeaderType.Dot;
          break;
        case "heavy":
          leader = wasm.TabLeaderType.Heavy;
          break;
        case "hyphen":
          leader = wasm.TabLeaderType.Hyphen;
          break;
        case "middleDot":
          leader = wasm.TabLeaderType.MiddleDot;
          break;
        case "none":
          leader = wasm.TabLeaderType.None;
          break;
        case "underscore":
          leader = wasm.TabLeaderType.None;
          break;
      }
      target = target.add_tab(val, leader, tab.pos ?? undefined) as T;
    }
  }

  if (property.frameProperty) {
    if (property.frameProperty?.h != null) {
      target = target.frame_height(property.frameProperty.h) as T;
    }
    if (property.frameProperty?.hRule != null) {
      target = target.h_rule(property.frameProperty.hRule) as T;
    }
    if (property.frameProperty?.hAnchor != null) {
      target = target.h_anchor(property.frameProperty.hAnchor) as T;
    }
    if (property.frameProperty?.hSpace != null) {
      target = target.h_space(property.frameProperty.hSpace) as T;
    }
    if (property.frameProperty?.vAnchor != null) {
      target = target.v_anchor(property.frameProperty.vAnchor) as T;
    }
    if (property.frameProperty?.vSpace != null) {
      target = target.v_space(property.frameProperty.vSpace) as T;
    }
    if (property.frameProperty?.w != null) {
      target = target.frame_width(property.frameProperty.w) as T;
    }
    if (property.frameProperty?.wrap != null) {
      target = target.wrap(property.frameProperty.wrap) as T;
    }
    if (property.frameProperty?.x != null) {
      target = target.frame_x(property.frameProperty.x) as T;
    }
    if (property.frameProperty?.xAlign != null) {
      target = target.x_align(property.frameProperty.xAlign) as T;
    }
    if (property.frameProperty?.y != null) {
      target = target.frame_y(property.frameProperty.y) as T;
    }
    if (property.frameProperty?.yAlign != null) {
      target = target.y_align(property.frameProperty.yAlign) as T;
    }
  }

  return target;
};

export const createParagraphProperty = (
  property: ParagraphProperty
): wasm.ParagraphProperty => {
  let p = new wasm.ParagraphProperty();
  const alignment = createParagraphAlignment(property.align);
  if (alignment != null) {
    p = p.align(alignment);
  }

  const textAlignment = createParagraphTextAlignment(property.textAlignment);
  if (textAlignment != null) {
    p = p.text_alignment(textAlignment);
  }

  if (property.adjustRightInd != null) {
    p = p.adjust_right_ind(property.adjustRightInd);
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
    p = p.indent(indent.left, kind, indent.specialIndentSize, indent.right);
  }

  if (typeof property.numbering !== "undefined") {
    const { numbering } = property;
    p = p.numbering(numbering.id, numbering.level);
  }

  // if (property.runProperty.bold) {
  //   p = p.bold();
  // }
  //
  // if (property.runProperty.color) {
  //   p = p.color(property.runProperty.color);
  // }

  if (typeof property.lineSpacing !== "undefined") {
    const spacing = buildLineSpacing(property);
    if (spacing) {
      p = p.line_spacing(spacing);
    }
  }

  // if (property.runProperty.italic) {
  //   p = p.italic();
  // }
  //
  // if (property.runProperty.size) {
  //   p = p.size(property.runProperty.size);
  // }

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
    // p = p.fonts(f);
  }

  if (property.keepLines) {
    p = p.keep_lines(true);
  }

  if (property.snapToGrid != null) {
    p = p.snap_to_grid(!!property.snapToGrid);
  }

  if (property.keepNext) {
    p = p.keep_next(true);
  }

  if (property.pageBreakBefore) {
    p = p.page_break_before(true);
  }

  if (property.widowControl) {
    p = p.widow_control(true);
  }

  if (property.outlineLvl != null) {
    p = p.outline_lvl(property.outlineLvl);
  }

  if (property.tabs) {
    for (const tab of property.tabs) {
      let val: wasm.TabValueType | undefined;
      let leader: wasm.TabLeaderType | undefined;
      switch (tab.val) {
        case "bar":
          val = wasm.TabValueType.Bar;
          break;
        case "bar":
          val = wasm.TabValueType.Bar;
          break;
        case "center":
          val = wasm.TabValueType.Center;
          break;
        case "clear":
          val = wasm.TabValueType.Clear;
          break;
        case "decimal":
          val = wasm.TabValueType.Decimal;
          break;
        case "end":
          val = wasm.TabValueType.End;
          break;
        case "right":
          val = wasm.TabValueType.Right;
          break;
        case "num":
          val = wasm.TabValueType.Num;
          break;
        case "start":
          val = wasm.TabValueType.Start;
          break;
        case "left":
          val = wasm.TabValueType.Left;
          break;
      }

      switch (tab.leader) {
        case "dot":
          leader = wasm.TabLeaderType.Dot;
          break;
        case "heavy":
          leader = wasm.TabLeaderType.Heavy;
          break;
        case "hyphen":
          leader = wasm.TabLeaderType.Hyphen;
          break;
        case "middleDot":
          leader = wasm.TabLeaderType.MiddleDot;
          break;
        case "none":
          leader = wasm.TabLeaderType.None;
          break;
        case "underscore":
          leader = wasm.TabLeaderType.None;
          break;
      }
      p = p.add_tab(val, leader, tab.pos ?? undefined);
    }
  }

  // TODO:
  /*
  if (property.frameProperty) {
    if (property.frameProperty?.h != null) {
      p = p.frame_height(property.frameProperty.h);
    }
    if (property.frameProperty?.hRule != null) {
      p = p.h_rule(property.frameProperty.hRule);
    }
    if (property.frameProperty?.hAnchor != null) {
      p = p.h_anchor(property.frameProperty.hAnchor);
    }
    if (property.frameProperty?.hSpace != null) {
      p = p.h_space(property.frameProperty.hSpace);
    }
    if (property.frameProperty?.vAnchor != null) {
      p = p.v_anchor(property.frameProperty.vAnchor);
    }
    if (property.frameProperty?.vSpace != null) {
      p = p.v_space(property.frameProperty.vSpace);
    }
    if (property.frameProperty?.w != null) {
      p = p.frame_width(property.frameProperty.w);
    }
    if (property.frameProperty?.wrap != null) {
      p = p.wrap(property.frameProperty.wrap);
    }
    if (property.frameProperty?.x != null) {
      p = p.frame_x(property.frameProperty.x);
    }
    if (property.frameProperty?.xAlign != null) {
      p = p.x_align(property.frameProperty.xAlign);
    }
    if (property.frameProperty?.y != null) {
      p = p.frame_y(property.frameProperty.y);
    }
    if (property.frameProperty?.yAlign != null) {
      p = p.y_align(property.frameProperty.yAlign);
    }
  }
    */

  return p;
};
