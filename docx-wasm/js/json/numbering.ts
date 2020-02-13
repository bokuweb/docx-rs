import { ParagraphPropertyJSON } from "./paragraph";

export type AbstractNumberingJSON = {
  level: number;
  start: number;
  format: string;
  text: string;
  jc: string;
  pstyle: string | null;
  paragraphProperty: ParagraphPropertyJSON;
};

export type NumberingJSON = {
  id: number;
  abstractNumId: number;
};
