import { ParagraphJSON, BookmarkStartJSON, BookmarkEndJSON } from "./paragraph";
import { TableJSON } from "./table";
import { SectionPropertyJSON } from "./section-property";

export type DocumentChildJSON =
  | ParagraphJSON
  | TableJSON
  | BookmarkStartJSON
  | BookmarkEndJSON;

export type DocumentJSON = {
  children: DocumentChildJSON[];
  sectionProperty: SectionPropertyJSON;
  hasNumbering: boolean;
};
