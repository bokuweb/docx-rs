import { RunJSON, RunChildJSON } from "./run";

export type ParagraphChildJSON =
  | RunJSON
  | InsertJSON
  | DeleteJSON
  | BookmarkStartJSON
  | BookmarkEndJSON;

export type NumberingPropertyJSON = {
  id: number;
  level: number;
};

export type ParagraphPropertyJSON = {
  runProperty: RunChildJSON;
  style: string | null;
  numberingProperty: NumberingPropertyJSON | null;
  alignment: "left" | "center" | "right" | "justified" | "both";
};

export type ParagraphJSON = {
  type: "paragraph";
  data: {
    property: ParagraphPropertyJSON;
    children: ParagraphChildJSON[];
  };
};

export type InsertJSON = {
  type: "insert";
  data: {
    run: RunJSON;
    author: string;
    data: string;
  };
};

export type DeleteJSON = {
  type: "delete";
  data: {
    run: RunJSON;
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
