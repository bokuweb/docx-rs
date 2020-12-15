import { ParagraphJSON } from "../";

export type CommentJSON = {
  id: number;
  author: string;
  date: string;
  paragraph: ParagraphJSON;
  parentCommentId: number | null;
};

export type CommentRangeStartJSON = {
  id: number;
  comment: CommentJSON;
};

export type CommentRangeEndJSON = {
  id: number;
};
