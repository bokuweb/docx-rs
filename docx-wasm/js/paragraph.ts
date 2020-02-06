import { Run } from "./run";
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

export type AlignmentType = "center" | "left" | "right" | "justified";

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
};

export class Paragraph {
  children: ParagraphChild[] = [];
  property: ParagraphProperty = {};

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
    this.property.numbering = { id, level };
    return this;
  }
}
