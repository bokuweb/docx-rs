import { Run, RunFonts } from "./run";
import {
  createDefaultParagraphProperty,
  ParagraphProperty,
  LineSpacing,
  AlignmentType,
  SpecialIndentKind,
  ParagraphPropertyChange,
  setParagraphProperty,
} from "./paragraph-property";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";
import { Hyperlink } from "./hyperlink";

import * as wasm from "./pkg";

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

  delete(author: string, date: string) {
    this.property.runProperty.del = { author, date };
    return this;
  }

  insert(author: string, date: string) {
    this.property.runProperty.ins = { author, date };
    return this;
  }

  paragraphPropertyChange(propertyChange: ParagraphPropertyChange) {
    this.property.paragraphPropertyChange = propertyChange;
    return this;
  }

  build() {
    let paragraph = wasm.createParagraph();
    this.children.forEach((child) => {
      if (child instanceof Run) {
        const run = child.build();
        paragraph = paragraph.add_run(run);
      } else if (child instanceof Insert) {
        const insert = child.build();
        paragraph = paragraph.add_insert(insert);
      } else if (child instanceof Delete) {
        const del = child.build();
        paragraph = paragraph.add_delete(del);
      } else if (child instanceof Hyperlink) {
        const hyperlink = child.build();
        paragraph = paragraph.add_hyperlink(hyperlink);
      } else if (child instanceof BookmarkStart) {
        paragraph = paragraph.add_bookmark_start(child.id, child.name);
      } else if (child instanceof BookmarkEnd) {
        paragraph = paragraph.add_bookmark_end(child.id);
      } else if (child instanceof Comment) {
        const comment = child.build();
        paragraph = paragraph.add_comment_start(comment);
      } else if (child instanceof CommentEnd) {
        paragraph = paragraph.add_comment_end(child.id);
      }
    });

    paragraph = setParagraphProperty(paragraph, this.property);

    if (typeof this.property.styleId !== "undefined") {
      paragraph = paragraph.style(this.property.styleId);
    }

    if (this.property.runProperty.del) {
      paragraph = paragraph.delete(
        this.property.runProperty.del.author,
        this.property.runProperty.del.date
      );
    }

    if (this.property.runProperty.ins) {
      paragraph = paragraph.insert(
        this.property.runProperty.ins.author,
        this.property.runProperty.ins.date
      );
    }

    if (this.property.paragraphPropertyChange) {
      let change = wasm.createParagraphPropertyChange();
      change = change
        .author(this.property.paragraphPropertyChange._author)
        .date(this.property.paragraphPropertyChange._date);

      if (this.property.paragraphPropertyChange._property.numbering) {
        change = change.numbering(
          this.property.paragraphPropertyChange._property.numbering.id,
          this.property.paragraphPropertyChange._property.numbering.level
        );
      }
      // TODO: add style, indent, alignment
      paragraph = paragraph.paragraph_property_change(change);
    }

    return paragraph;
  }
}
