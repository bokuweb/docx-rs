import { Run, RunFonts } from "./run";
import {
  createDefaultParagraphProperty,
  ParagraphProperty,
  LineSpacing,
  AlignmentType,
  SpecialIndentKind,
  ParagraphPropertyChange,
} from "./paragraph-property";
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
    specialIndentSize?: number,
    right?: number
  ) {
    this.property.indent = {
      left,
      specialIndentKind,
      specialIndentSize,
      right,
    };
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

  characterSpacing(spacing: number) {
    this.property.runProperty.characterSpacing = spacing;
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

  delete(author: string, date: string) {
    this.property.runProperty.del = { author, date };
    return this;
  }

  insert(author: string, date: string) {
    this.property.runProperty.ins = { author, date };
    return this;
  }

  outlineLevel(v: number) {
    this.property = { ...this.property, outlineLvl: v };
    return this;
  }

  paragraphPropertyChange(propertyChange: ParagraphPropertyChange) {
    this.property.paragraphPropertyChange = propertyChange;
    return this;
  }
}
