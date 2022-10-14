import {
  BookmarkEndJSON,
  BookmarkStartJSON,
  CommentRangeEndJSON,
  CommentRangeStartJSON,
  ParagraphJSON,
  TableJSON,
} from "..";

export type StructuredTagJSON = {
  type: "structuredDataTag";
  data: {
    children: StructuredDataTagChildJSON[];
  };
};

export type StructuredDataTagChildJSON =
  | ParagraphJSON
  | TableJSON
  | CommentRangeStartJSON
  | CommentRangeEndJSON
  | BookmarkStartJSON
  | BookmarkEndJSON;
