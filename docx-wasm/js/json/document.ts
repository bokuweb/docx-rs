import { ParagraphJSON } from "./paragraph";

export type DocumentChildJSON = ParagraphJSON;

export type DocumentJSON = {
  children: DocumentChildJSON[];
  sectionProperty: {
    pageSize: {
      w: number;
      h: number;
    };
    pageMargin: {
      top: number;
      left: number;
      bottom: number;
      right: number;
      header: number;
      footer: number;
      gutter: number;
    };
    columns: number;
    documentGrid: number;
  };
  hasNumbering: boolean;
};
