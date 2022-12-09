import * as wasm from "./pkg";

import { Run } from "./run";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";
import { ParagraphChild } from "./paragraph";

export type HyperlinkType = "anchor" | "external";

export class Hyperlink {
  v: string;
  type: HyperlinkType;
  children: ParagraphChild[] = [];

  constructor(v: string, t: HyperlinkType) {
    this.v = v;
    this.type = t;
  }

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

  build() {
    let hyperlink = wasm.createHyperlink(this.v, convertHyperlinkType(this));

    this.children.forEach((child) => {
      if (child instanceof Run) {
        const run = child.build();
        hyperlink = hyperlink.add_run(run);
      } else if (child instanceof Insert) {
        const insert = child.build();
        hyperlink = hyperlink.add_insert(insert);
      } else if (child instanceof Delete) {
        const del = child.build();
        hyperlink = hyperlink.add_delete(del);
      } else if (child instanceof BookmarkStart) {
        hyperlink = hyperlink.add_bookmark_start(child.id, child.name);
      } else if (child instanceof BookmarkEnd) {
        hyperlink = hyperlink.add_bookmark_end(child.id);
      } else if (child instanceof Comment) {
        const comment = child.build();
        hyperlink = hyperlink.add_comment_start(comment);
      } else if (child instanceof CommentEnd) {
        hyperlink = hyperlink.add_comment_end(child.id);
      }
    });

    return hyperlink;
  }
}

export const convertHyperlinkType = (link: Hyperlink): wasm.HyperlinkType => {
  if (link.type === "anchor") {
    return wasm.HyperlinkType.Anchor;
  }
  return wasm.HyperlinkType.External;
};
