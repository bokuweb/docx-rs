import { ParagraphJSON } from "./paragraph";
import { TableJSON } from "./table";

export type DocumentChildJSON = ParagraphJSON | TableJSON;

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
