import { Run } from "./run";
import { RunFonts } from "./run-property";
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
import { TextAlignmentType } from "./json/bindings/TextAlignmentType";
import { TabValueType } from "./json/bindings/TabValueType";
import { TabLeaderType } from "./json/bindings/TabLeaderType";
import { NumPages } from "./num-pages";
import { PageNum } from "./page-num";

export type ParagraphChild =
  | Run
  | Insert
  | Delete
  | Hyperlink
  | BookmarkStart
  | BookmarkEnd
  | Comment
  | CommentEnd
  | NumPages
  | PageNum;

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

  addPageNum() {
    this.children.push(new PageNum());
    return this;
  }

  addNumPages() {
    this.children.push(new NumPages());
    return this;
  }

  tabs(
    tabs: {
      val: TabValueType | null;
      leader: TabLeaderType | null;
      pos: number | null;
    }[]
  ) {
    this.property._tabs = tabs;
    return this;
  }

  align(type: AlignmentType) {
    this.property._align = type;
    return this;
  }

  textAlignment(type: TextAlignmentType) {
    this.property._textAlignment = type;
    return this;
  }

  adjustRightInd(v: number) {
    this.property._adjustRightInd = v;
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
    this.property.runProperty.spacing(spacing);
    return this;
  }

  snapToGrid(v: boolean) {
    this.property.snapToGrid = v;
    return this;
  }

  keepNext(v: boolean) {
    this.property.keepNext = v;
    return this;
  }

  keepLines(v: boolean) {
    this.property.keepLines = v;
    return this;
  }

  pageBreakBefore(v: boolean) {
    this.property.pageBreakBefore = v;
    return this;
  }

  widowControl(v: boolean) {
    this.property.widowControl = v;
    return this;
  }

  // run property
  size(size: number) {
    this.property.runProperty.size(size);
    return this;
  }

  color(color: string) {
    this.property.runProperty.color(color);
    return this;
  }

  bold() {
    this.property.runProperty.bold();
    return this;
  }

  italic() {
    this.property.runProperty.italic();
    return this;
  }

  fonts(fonts: RunFonts) {
    this.property.runProperty.fonts(fonts);
    return this;
  }

  delete(author: string, date: string) {
    this.property.runProperty.delete(author, date);
    return this;
  }

  insert(author: string, date: string) {
    this.property.runProperty.insert(author, date);
    return this;
  }

  outlineLevel(v: number) {
    this.property.outlineLvl = v;
    return this;
  }

  paragraphPropertyChange(propertyChange: ParagraphPropertyChange) {
    this.property.paragraphPropertyChange = propertyChange;
    return this;
  }

  // frameProperty
  frameHeight(h: number) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.h = h;
    return this;
  }
  hRule(r: string) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.hRule = r;
    return this;
  }

  hAnchor(a: string) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.hAnchor = a;
    return this;
  }

  hSpace(s: number) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.hSpace = s;
    return this;
  }

  vAnchor(a: string) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.vAnchor = a;
    return this;
  }

  vSpace(s: number) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.vSpace = s;
    return this;
  }

  frameWidth(w: number) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.w = w;
    return this;
  }

  wrap(w: string) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.wrap = w;
    return this;
  }

  frameX(x: number) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.x = x;
    return this;
  }

  xAlign(a: string) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.xAlign = a;
    return this;
  }

  frameY(y: number) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.y = y;
    return this;
  }

  yAlign(y: string) {
    this.property.frameProperty = { ...this.property.frameProperty };
    this.property.frameProperty.yAlign = y;
    return this;
  }
}
