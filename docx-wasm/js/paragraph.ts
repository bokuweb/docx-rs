import { Run, RunProperty, RunFonts, createDefaultRunProperty } from "./run";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";
import { Hyperlink } from "./hyperlink";

export type ParagraphChild =
  | Run
  | Insert
  | Delete
  | Hyperlink
  | BookmarkStart
  | BookmarkEnd
  | Comment
  | CommentEnd;

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

export class Paragraph {
  hasNumberings = false;
  children: ParagraphChild[] = [];
  property: ParagraphProperty = createDefaultParagraphProperty();

  addRun(run: Run) {
    this.children.push(run);
    return this;
  }

  addHyperlink(link: Hyperlink) {
    this.children.push(link);
    return this;
  }

  addInsert(ins: Insert) {
    this.children.push(ins);
    return this;
  }

  addDelete(del: Delete) {
    this.children.push(del);
    return this;
  }

  addBookmarkStart(id: number, name: string) {
    this.children.push(new BookmarkStart(id, name));
    return this;
  }

  addBookmarkEnd(id: number) {
    this.children.push(new BookmarkEnd(id));
    return this;
  }

  addCommentStart(comment: Comment) {
    this.children.push(comment);
    return this;
  }

  addCommentEnd(end: CommentEnd) {
    this.children.push(end);
    return this;
  }

  align(type: AlignmentType) {
    this.property.align = type;
    return this;
  }

  style(id: string) {
    this.property.styleId = id;
    return this;
  }

  indent(
    left: number,
    specialIndentKind?: SpecialIndentKind,
    specialIndentSize?: number
  ) {
    this.property.indent = { left, specialIndentKind, specialIndentSize };
    return this;
  }

  numbering(id: number, level: number) {
    this.hasNumberings = true;
    this.property.numbering = { id, level };
    return this;
  }

  lineSpacing(spacing: LineSpacing) {
    this.property.lineSpacing = spacing;
    return this;
  }

  keepNext(v: boolean) {
    this.property = { ...this.property, keepNext: v };
    return this;
  }

  keepLines(v: boolean) {
    this.property = { ...this.property, keepLines: v };
    return this;
  }

  pageBreakBefore(v: boolean) {
    this.property = { ...this.property, pageBreakBefore: v };
    return this;
  }

  widowControl(v: boolean) {
    this.property = { ...this.property, widowControl: v };
    return this;
  }

  // run property
  size(size: number) {
    this.property.runProperty = { ...this.property.runProperty, size };
    return this;
  }

  bold() {
    this.property.runProperty = { ...this.property.runProperty, bold: true };
    return this;
  }

  italic() {
    this.property.runProperty = { ...this.property.runProperty, italic: true };
    return this;
  }

  fonts(fonts: RunFonts) {
    this.property.runProperty = { ...this.property.runProperty, fonts };
    return this;
  }
}
