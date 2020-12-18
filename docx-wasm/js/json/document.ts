import { ParagraphJSON, BookmarkStartJSON, BookmarkEndJSON } from "./paragraph";
import { TableJSON } from "./table";
import { SectionPropertyJSON } from "./section-property";
import { CommentRangeStartJSON, CommentRangeEndJSON } from "..";

export type DocumentChildJSON =
  | ParagraphJSON
  | TableJSON
  | CommentRangeStartJSON
  | CommentRangeEndJSON
  | BookmarkStartJSON
  | BookmarkEndJSON;

export type DocumentJSON = {
  children: DocumentChildJSON[];
  sectionProperty: SectionPropertyJSON;
  hasNumbering: boolean;
};
