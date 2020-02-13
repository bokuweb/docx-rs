import { ParagraphJSON } from "./paragraph";

export type TableCellChildJSON = ParagraphJSON;

export type TableCellPropertyJSON = {
  width: {
    width: number;
    widthType: "DXA" | "auto" | "pct";
  } | null;
  borders: any | null;
  gridSpan: number | null;
  verticalMerge: "restart" | "continue" | null;
  hasNumbering: boolean;
};

export type TableCellJSON = {
  children: TableCellChildJSON[];
  property: TableCellPropertyJSON;
};

export type TableRowJSON = {
  cells: TableCellJSON[];
};

export type TablePropertyJSON = {
  width: {
    width: number;
    widthType: "DXA" | "auto" | "pct";
  } | null;
  borders: any | null;
  gridSpan: number | null;
  verticalMerge: "restart" | "continue" | null;
  hasNumbering: boolean;
};

export type TableJSON = {
  type: "table";
  data: {
    rows: TableRowJSON[];
    grid: number[];
    hasNumbering: boolean;
    property: TablePropertyJSON;
  };
};
