import { RunProperty, createDefaultRunProperty } from "./run-property";

import * as wasm from "./pkg";
import { TextAlignmentType } from "./json/bindings/TextAlignmentType";
import { Tab } from "./json/bindings/Tab";
import { AlignmentType } from "./json/bindings/AlignmentType";
import { TabValueType } from "./json/bindings/TabValueType";
import { TabLeaderType } from "./json/bindings/TabLeaderType";

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

export class ParagraphProperty {
  _align?: AlignmentType;
  _textAlignment?: TextAlignmentType;
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
  runProperty: RunProperty = createDefaultRunProperty();
  keepNext: boolean;
  keepLines: boolean;
  pageBreakBefore: boolean;
  widowControl: boolean;
  paragraphPropertyChange?: ParagraphPropertyChange;
  outlineLvl?: number | null;
  _snapToGrid?: boolean;
  _adjustRightInd?: number;
  _tabs?: Tab[];
  frameProperty?: FrameProperty;

  constructor() {}

  tabs(
    tabs: {
      val: TabValueType | null;
      leader: TabLeaderType | null;
      pos: number | null;
    }[]
  ) {
    this._tabs = tabs;
    return this;
  }

  align(type: AlignmentType) {
    this._align = type;
    return this;
  }

  textAlignment(type: TextAlignmentType) {
    this._textAlignment = type;
    return this;
  }

  adjustRightInd(v: number) {
    this._adjustRightInd = v;
    return this;
  }

  snapToGrid(v: boolean) {
    this._snapToGrid = v;
    return this;
  }

  style(id: string) {
    this.styleId = id;
    return this;
  }
}

export const createDefaultParagraphProperty = (): ParagraphProperty => {
  let p = new ParagraphProperty();
  p.runProperty = createDefaultRunProperty();
  p.keepNext = false;
  p.keepLines = false;
  p.pageBreakBefore = false;
  p.widowControl = false;
  return p;
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
    this._property._align = type;
    return this;
  }

  textAlignment(type: TextAlignmentType) {
    this._property._textAlignment = type;
    return this;
  }

  adjustRightInd(v: number) {
    this._property._adjustRightInd = v;
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
  const alignment = createParagraphAlignment(property._align);
  if (alignment != null) {
    target = target.align(alignment) as T;
  }

  const textAlignment = createParagraphTextAlignment(property._textAlignment);
  if (textAlignment != null) {
    target = target.text_alignment(textAlignment) as T;
  }

  if (property._adjustRightInd != null) {
    target = target.adjust_right_ind(property._adjustRightInd) as T;
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

  if (property.runProperty._bold) {
    target = target.bold() as T;
  }

  if (property.runProperty._color) {
    target = target.color(property.runProperty._color) as T;
  }

  if (typeof property.lineSpacing !== "undefined") {
    const spacing = buildLineSpacing(property);
    if (spacing) {
      target = target.line_spacing(spacing) as T;
    }
  }

  if (property.runProperty._italic) {
    target = target.italic() as T;
  }

  if (property.runProperty._size) {
    target = target.size(property.runProperty._size) as T;
  }

  if (property.runProperty._fonts) {
    let f = wasm.createRunFonts();
    if (property.runProperty._fonts._ascii) {
      f = f.ascii(property.runProperty._fonts._ascii);
    }
    if (property.runProperty._fonts._hiAnsi) {
      f = f.hi_ansi(property.runProperty._fonts._hiAnsi);
    }
    if (property.runProperty._fonts._cs) {
      f = f.cs(property.runProperty._fonts._cs);
    }
    if (property.runProperty._fonts._eastAsia) {
      f = f.east_asia(property.runProperty._fonts._eastAsia);
    }
    target = target.fonts(f) as T;
  }

  if (property.keepLines) {
    target = target.keep_lines(true) as T;
  }

  if (property._snapToGrid != null) {
    target = target.snap_to_grid(!!property._snapToGrid) as T;
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

  if (property._tabs) {
    for (const tab of property._tabs) {
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
  let p = wasm.createParagraphProperty();
  const alignment = createParagraphAlignment(property._align);
  if (alignment != null) {
    p = p.align(alignment);
  }

  const textAlignment = createParagraphTextAlignment(property._textAlignment);
  if (textAlignment != null) {
    p = p.text_alignment(textAlignment);
  }

  if (property._adjustRightInd != null) {
    p = p.adjust_right_ind(property._adjustRightInd);
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

  if (typeof property.lineSpacing !== "undefined") {
    const spacing = buildLineSpacing(property);
    if (spacing) {
      p = p.line_spacing(spacing);
    }
  }

  if (typeof property.numbering !== "undefined") {
    const { numbering } = property;
    p = p.numbering(numbering.id, numbering.level);
  }

  let runProperty = wasm.createRunProperty();
  if (property.runProperty._bold) {
    runProperty = runProperty.bold();
  }

  if (property.runProperty._color) {
    runProperty = runProperty.color(property.runProperty._color);
  }

  if (property.runProperty._italic) {
    runProperty = runProperty.italic();
  }

  if (property.runProperty._size) {
    runProperty = runProperty.size(property.runProperty._size);
  }

  if (property.runProperty._fonts) {
    let f = wasm.createRunFonts();
    if (property.runProperty._fonts._ascii) {
      f = f.ascii(property.runProperty._fonts._ascii);
    }
    if (property.runProperty._fonts._hiAnsi) {
      f = f.hi_ansi(property.runProperty._fonts._hiAnsi);
    }
    if (property.runProperty._fonts._cs) {
      f = f.cs(property.runProperty._fonts._cs);
    }
    if (property.runProperty._fonts._eastAsia) {
      f = f.east_asia(property.runProperty._fonts._eastAsia);
    }
    runProperty = runProperty.fonts(f);
  }

  if (property.runProperty) {
    p = p.run_property(runProperty);
  }

  if (property.keepLines) {
    p = p.keep_lines(true);
  }

  if (property._snapToGrid != null) {
    p = p.snap_to_grid(!!property._snapToGrid);
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

  if (property.styleId) {
    p = p.style(property.styleId);
  }

  if (property._tabs) {
    for (const tab of property._tabs) {
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

  if (property.frameProperty) {
    let frameProperty = wasm.createFrameProperty();
    if (property.frameProperty?.h != null) {
      frameProperty = frameProperty.height(property.frameProperty.h);
    }
    if (property.frameProperty?.hRule != null) {
      frameProperty = frameProperty.h_rule(property.frameProperty.hRule);
    }
    if (property.frameProperty?.hAnchor != null) {
      frameProperty = frameProperty.h_anchor(property.frameProperty.hAnchor);
    }
    if (property.frameProperty?.hSpace != null) {
      frameProperty = frameProperty.h_space(property.frameProperty.hSpace);
    }
    if (property.frameProperty?.vAnchor != null) {
      frameProperty = frameProperty.v_anchor(property.frameProperty.vAnchor);
    }
    if (property.frameProperty?.vSpace != null) {
      frameProperty = frameProperty.v_space(property.frameProperty.vSpace);
    }
    if (property.frameProperty?.w != null) {
      frameProperty = frameProperty.width(property.frameProperty.w);
    }
    if (property.frameProperty?.wrap != null) {
      frameProperty = frameProperty.wrap(property.frameProperty.wrap);
    }
    if (property.frameProperty?.x != null) {
      frameProperty = frameProperty.x(property.frameProperty.x);
    }
    if (property.frameProperty?.xAlign != null) {
      frameProperty = frameProperty.x_align(property.frameProperty.xAlign);
    }
    if (property.frameProperty?.y != null) {
      frameProperty = frameProperty.y(property.frameProperty.y);
    }
    if (property.frameProperty?.yAlign != null) {
      frameProperty = frameProperty.y_align(property.frameProperty.yAlign);
    }
    p = p.frame_property(frameProperty);
  }

  return p;
};
