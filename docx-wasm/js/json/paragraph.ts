import { RunJSON, RunChildJSON, RunPropertyJSON } from "./run";
import { IndentJSON } from "./indent";
import { CommentRangeStartJSON, CommentRangeEndJSON } from "..";

export type ParagraphChildJSON =
  | RunJSON
  | InsertJSON
  | DeleteJSON
  | CommentRangeStartJSON
  | CommentRangeEndJSON
  | BookmarkStartJSON
  | BookmarkEndJSON;

export type NumberingPropertyJSON = {
  id: number | null;
  level: number | null;
};

export type ParagraphPropertyJSON = {
  runProperty: RunPropertyJSON;
  style: string | null;
  numberingProperty: NumberingPropertyJSON | null;
  alignment: "left" | "center" | "right" | "justified" | "both";
  indent: IndentJSON | null;
  lineHeight: number | null;
};

export type ParagraphJSON = {
  type: "paragraph";
  data: {
    id: string;
    property: ParagraphPropertyJSON;
    children: ParagraphChildJSON[];
  };
};

export type InsertJSON = {
  type: "insert";
  data: {
    children: (
      | DeleteJSON
      | RunJSON
      | CommentRangeStartJSON
      | CommentRangeEndJSON
    )[];
    author: string;
    data: string;
  };
};

export type DeleteJSON = {
  type: "delete";
  data: {
    runs: {
      runProperty: RunPropertyJSON;
      children: RunChildJSON[];
    }[];
    author: string;
    data: string;
  };
};

export type BookmarkStartJSON = {
  type: "bookmarkStart";
  data: {
    id: number;
    name: string;
  };
};

export type BookmarkEndJSON = {
  type: "bookmarkEnd";
  data: {
    id: number;
  };
};
