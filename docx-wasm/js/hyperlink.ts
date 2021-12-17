import { Run } from "./run";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";
import { ParagraphChild } from "./paragraph";

export class Hyperlink {
  _rid?: string;
  _anchor?: string;
  _history: boolean = false;
  children: ParagraphChild[] = [];

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

  rid(rid: string) {
    this._rid = rid;
    return this;
  }

  anchor(anchor: string) {
    this._anchor = anchor;
    return this;
  }

  history() {
    this._history = true;
    return this;
  }
}
