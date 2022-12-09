import { Paragraph } from "./paragraph";
import { Table } from "./table";

import * as wasm from "./pkg";

export class Comment {
  id: number;
  _author: string;
  _date: string;
  children: (Paragraph | Table)[] = [];
  _parentCommentId: number;

  constructor(id: number) {
    this.id = id;
  }

  author(author: string) {
    this._author = author;
    return this;
  }

  date(date: string) {
    this._date = date;
    return this;
  }

  addParagraph(p: Paragraph) {
    this.children.push(p);
    return this;
  }

  parentCommentId(id: number) {
    this._parentCommentId = id;
    return this;
  }

  build() {
    let comment = wasm.createComment(this.id);
    this.children.forEach((child) => {
      if (child instanceof Paragraph) {
        comment = comment.add_paragraph(child.build());
      } else if (child instanceof Table) {
        // TODO: Support later
      }
    });
    if (this._author) {
      comment = comment.author(this._author);
    }
    if (this._date) {
      comment = comment.date(this._date);
    }
    if (this._parentCommentId) {
      comment = comment.parent_comment_id(this._parentCommentId);
    }
    return comment;
  }
}
