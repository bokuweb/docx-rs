import { RunJSON, RunPropertyJSON } from "./run";
import { IndentJSON } from "./indent";
import {
  CommentRangeStartJSON,
  CommentRangeEndJSON,
  SectionPropertyJSON,
} from "..";
import { LineSpacingJSON } from "./line_spacing";

export type ParagraphChildJSON =
  | RunJSON
  | InsertJSON
  | DeleteJSON
  | HyperlinkJSON
  | CommentRangeStartJSON
  | CommentRangeEndJSON
  | BookmarkStartJSON
  | BookmarkEndJSON;

export type HyperlinkChildJSON =
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
  style?: string | null;
  numberingProperty?: NumberingPropertyJSON | null;
  alignment?: "left" | "center" | "right" | "justified" | "both";
  indent?: IndentJSON | null;
  lineSpacing?: LineSpacingJSON | null;
  divId?: string | null;
  keepNext?: boolean;
  keepLines?: boolean;
  pageBreakBefore?: boolean;
  widowControl?: boolean;
  outlineLvl?: number | null;
  paragraphPropertyChange?: {
    author: string;
    date: string;
    property: ParagraphPropertyJSON;
  };
  sectionProperty?: SectionPropertyJSON;
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
    date: string;
  };
};

export type DeleteJSON = {
  type: "delete";
  data: {
    children: DeleteChildJSON[];
    author: string;
    date: string;
  };
};

export type HyperlinkJSON = {
  type: "hyperlink";
  data:
    | {
        type: "external";
        rid: string;
        children: HyperlinkChildJSON[];
        history: number | null;
      }
    | {
        type: "anchor";
        anchor: string;
        children: HyperlinkChildJSON[];
      };
};

export type DeleteChildJSON =
  | RunJSON
  | CommentRangeStartJSON
  | CommentRangeEndJSON;

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
