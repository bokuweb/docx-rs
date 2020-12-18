import { ParagraphJSON, TableJSON } from "../";

export type CommentChildJSON = ParagraphJSON | TableJSON;

export type CommentJSON = {
  id: number;
  author: string;
  date: string;
  children: CommentChildJSON[];
  parentCommentId: number | null;
};

export type CommentRangeStartJSON = {
  type: "commentRangeStart";
  data: {
    id: number;
    comment: CommentJSON;
  };
};

export type CommentRangeEndJSON = {
  type: "commentRangeEnd";
  data: {
    id: number;
  };
};
