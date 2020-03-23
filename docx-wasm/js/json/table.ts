import { ParagraphJSON } from "./paragraph";
import { BorderJSON } from "./border";

export type TableCellChildJSON = ParagraphJSON;

export type WidthType = "DXA" | "Auto" | "Pct";

export type TableCellPropertyJSON = {
  width: {
    width: number;
    widthType: WidthType;
  } | null;
  borders: any | null;
  gridSpan: number | null;
  verticalMerge: "restart" | "continue" | null;
  verticalAlign: "top" | "center" | "bottom" | null;
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
    widthType: WidthType;
  } | null;
  justification: "left" | "center" | "right";
  borders: {
    top: BorderJSON;
    left: BorderJSON;
    bottom: BorderJSON;
    right: BorderJSON;
    insideH: BorderJSON;
    insideV: BorderJSON;
  } | null;
  margins: {
    top: number;
    left: number;
    bottom: number;
    right: number;
  } | null;
  indent: {
    width: number;
    widthType: WidthType;
  } | null;
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
