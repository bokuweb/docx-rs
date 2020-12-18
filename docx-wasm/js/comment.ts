import { Paragraph } from "./paragraph";
import { Table } from "./table";

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
}
