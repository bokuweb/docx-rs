import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";
import { Paragraph } from "./paragraph";
import { Table } from "./table";
import { Run } from "./run";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Hyperlink, convertHyperlinkType } from "./hyperlink";
import { setParagraphProperty } from "./paragraph-property";

import * as wasm from "./pkg";

type Child = Paragraph | Table | Comment | Hyperlink;

function buildHyperlink(child: Hyperlink) {
  let hyperlink = wasm.createHyperlink(child.v, convertHyperlinkType(child));

  child.children.forEach((child) => {
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
      hyperlink = hyperlink.add_comment_start(build(child));
    } else if (child instanceof CommentEnd) {
      hyperlink = hyperlink.add_comment_end(child.id);
    }
  });

  return hyperlink;
}

function buildParagraph(child: Paragraph) {
  let paragraph = wasm.createParagraph();
  child.children.forEach((child) => {
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
      paragraph = paragraph.add_hyperlink(build(child));
    } else if (child instanceof BookmarkStart) {
      paragraph = paragraph.add_bookmark_start(child.id, child.name);
    } else if (child instanceof BookmarkEnd) {
      paragraph = paragraph.add_bookmark_end(child.id);
    } else if (child instanceof Comment) {
      const comment = build(child);
      paragraph = paragraph.add_comment_start(comment as wasm.Comment);
    } else if (child instanceof CommentEnd) {
      paragraph = paragraph.add_comment_end(child.id);
    }
  });

  paragraph = setParagraphProperty(paragraph, child.property);

  if (typeof child.property.styleId !== "undefined") {
    paragraph = paragraph.style(child.property.styleId);
  }

  if (child.property.runProperty.del) {
    paragraph = paragraph.delete(
      child.property.runProperty.del.author,
      child.property.runProperty.del.date
    );
  }

  if (child.property.runProperty.ins) {
    paragraph = paragraph.insert(
      child.property.runProperty.ins.author,
      child.property.runProperty.ins.date
    );
  }

  if (child.property.runProperty.characterSpacing != null) {
    paragraph = paragraph.character_spacing(
      child.property.runProperty.characterSpacing
    );
  }

  if (child.property.paragraphPropertyChange) {
    let change = wasm.createParagraphPropertyChange();
    change = change
      .author(child.property.paragraphPropertyChange._author)
      .date(child.property.paragraphPropertyChange._date);

    if (child.property.paragraphPropertyChange._property.numbering) {
      change = change.numbering(
        child.property.paragraphPropertyChange._property.numbering.id,
        child.property.paragraphPropertyChange._property.numbering.level
      );
    }
    // TODO: add style, indent, alignment
    paragraph = paragraph.paragraph_property_change(change);
  }

  return paragraph;
}

function buildComment(child: Comment) {
  let comment = wasm.createComment(child.id);
  child.children.forEach((c) => {
    if (c instanceof Paragraph) {
      comment = comment.add_paragraph(buildParagraph(c));
    } else if (child instanceof Table) {
      // TODO: Support later
    }
  });
  if (child._author) {
    comment = comment.author(child._author);
  }
  if (child._date) {
    comment = comment.date(child._date);
  }
  if (child._parentCommentId) {
    comment = comment.parent_comment_id(child._parentCommentId);
  }
  return comment;
}

export function build<T>(child: Child) {
  if (child instanceof Comment) {
    return buildComment(child) as T;
  } else if (child instanceof Paragraph) {
    return buildParagraph(child) as T;
  } else if (child instanceof Hyperlink) {
    return buildHyperlink(child) as T;
  }
  throw new Error(`not found builder for child: ${child}`);
}
