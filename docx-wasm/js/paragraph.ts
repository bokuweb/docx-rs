import { Run, RunProperty, RunFonts } from "./run";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";

export type ParagraphChild =
  | Run
  | Insert
  | Delete
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
  runProperty: RunProperty;
};

export class Paragraph {
  hasNumberings = false;
  children: ParagraphChild[] = [];
  property: ParagraphProperty = {
    runProperty: {},
  };

  addRun(run: Run) {
    this.children.push(run);
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
