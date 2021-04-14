import { ParagraphJSON } from "./paragraph";
import { BorderJSON } from "./border";
import { HeightRule } from "../table-row";
import { TextDirectionType } from "../table-cell";
import { ShadingJSON } from "./shading";
import { TableLayoutType } from "../table";

export type TableCellChildJSON = ParagraphJSON;

export type WidthType = "DXA" | "Auto" | "Pct" | "nil";
export { TextDirectionType } from "../table-cell";

export { HeightRule } from "../table-row";

export type TableCellPropertyJSON = {
  width: {
    width: number;
    widthType: WidthType;
  } | null;
  borders: any | null;
  gridSpan: number | null;
  verticalMerge: "restart" | "continue" | null;
  verticalAlign: "top" | "center" | "bottom" | null;
  textDirection: TextDirectionType | null;
  hasNumbering: boolean;
  shading: ShadingJSON | null;
};

export type TableRowPropertyJSON = {
  gridAfter: number | null;
  rowHeight: number | null;
  heightRule: HeightRule | null;
  widthAfter: number | null;
};

export type TableCellJSON = {
  children: TableCellChildJSON[];
  property: TableCellPropertyJSON;
};

export type TableRowJSON = {
  cells: TableCellJSON[];
  property: TableRowPropertyJSON;
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
    top: { val: number; widthType: WidthType };
    left: { val: number; widthType: WidthType };
    bottom: { val: number; widthType: WidthType };
    right: { val: number; widthType: WidthType };
  } | null;
  indent: {
    width: number;
    widthType: WidthType;
  };
  style: string | null;
  layout: TableLayoutType | null;
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
