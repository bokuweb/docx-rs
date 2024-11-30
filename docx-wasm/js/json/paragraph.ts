import { RunJSON, RunPropertyJSON } from "./run";
import { IndentJSON } from "./indent";
import {
  CommentRangeEndJSON,
  CommentRangeStartJSON,
  SectionPropertyJSON,
} from "..";
import { LineSpacingJSON } from "./line_spacing";
import { FrameProperty as FramePropertyJSON } from "./bindings/FrameProperty";
import { TextAlignmentType } from "./bindings/TextAlignmentType";
import { AlignmentType } from "./bindings/AlignmentType";

export { FrameProperty as FramePropertyJSON } from "./bindings/FrameProperty";

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

export type CustomTabStopType =
  | "bar"
  | "center"
  | "clear"
  | "decimal"
  | "end"
  | "right"
  | "num"
  | "start"
  | "left";

export type CustomTabStopJSON = {
  val: CustomTabStopType | null;
  leader: string | null;
  pos: number | null;
};

export type NumberingPropertyJSON = {
  id: number | null;
  level: number | null;
};

export type ParagraphPropertyJSON = {
  runProperty?: RunPropertyJSON;
  style?: string | null;
  numberingProperty?: NumberingPropertyJSON | null;
  alignment?: AlignmentType;
  textAlignment?: TextAlignmentType;
  adjustRightInd?: number;
  indent?: IndentJSON | null;
  lineSpacing?: LineSpacingJSON | null;
  divId?: string | null;
  keepNext?: boolean;
  keepLines?: boolean;
  snapToGrid?: boolean;
  pageBreakBefore?: boolean;
  widowControl?: boolean;
  outlineLvl?: number | null;
  paragraphPropertyChange?: {
    author: string;
    date: string;
    property: ParagraphPropertyJSON;
  };
  sectionProperty?: SectionPropertyJSON;
  tabs: CustomTabStopJSON[];
  frameProperty?: FramePropertyJSON;
};

export type ParagraphJSON = {
  type: "paragraph";
  data: {
    id: string;
    property: ParagraphPropertyJSON;
    children: ParagraphChildJSON[];
  };
};

export type InsertJSONData = {
  children: (
    | DeleteJSON
    | RunJSON
    | CommentRangeStartJSON
    | CommentRangeEndJSON
  )[];
  author: string;
  date: string;
};

export type InsertJSON = {
  type: "insert";
  data: InsertJSONData;
};

export type DeleteJSONData = {
  children: DeleteChildJSON[];
  author: string;
  date: string;
};

export type DeleteJSON = {
  type: "delete";
  data: DeleteJSONData;
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
