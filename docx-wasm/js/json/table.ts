import { ParagraphJSON } from "./paragraph";
import { BorderJSON } from "./border";
import { HeightRule } from "../table-row";
import { TextDirectionType } from "../table-cell";
import { ShadingJSON } from "./shading";
import { TableLayoutType } from "../table";
import { DeleteJSONData, InsertJSONData, TableCellBordersJSON } from "..";
import { StructuredTagJSON } from "./structured-data-tag";

import { TablePositionProperty as TablePositionPropertyJSON } from "./bindings/TablePositionProperty";

export { TablePositionProperty as TablePositionPropertyJSON } from "./bindings/TablePositionProperty";
export { TableCellBorder as TableCellBorderJSON } from "./bindings/TableCellBorder";

export { TableCellBorders as TableCellBordersJSON } from "./bindings/TableCellBorders";

export type TableCellChildJSON = ParagraphJSON | TableJSON | StructuredTagJSON;

export type WidthType = "dxa" | "auto" | "pct" | "nil";

export { TextDirectionType } from "../table-cell";

export { HeightRule } from "../table-row";

export type TableCellPropertyJSON = {
  width: {
    width: number;
    widthType: WidthType;
  } | null;
  borders: TableCellBordersJSON | null;
  gridSpan: number | null;
  verticalMerge: "restart" | "continue" | null;
  verticalAlign: "top" | "center" | "bottom" | null;
  textDirection: TextDirectionType | null;
  shading: ShadingJSON | null;
  margins?: CellMarginsJSON;
};

export type TableRowPropertyJSON = {
  gridAfter: number | null;
  gridBefore: number | null;
  rowHeight: number | null;
  heightRule: HeightRule | null;
  widthAfter: number | null;
  widthBefore: number | null;
  del?: DeleteJSONData;
  ins?: InsertJSONData;
};

export type TableCellJSON = {
  type: "tableCell";
  data: {
    children: TableCellChildJSON[];
    property: TableCellPropertyJSON;
  };
};

export type TableRowJSON = {
  type: "tableRow";
  data: {
    cells: TableCellJSON[];
    property: TableRowPropertyJSON;
  };
};

export type TableCellMarginJSON = { val: number; widthType: WidthType };

export type TableCellMarginsJSON = {
  top: TableCellMarginJSON;
  left: TableCellMarginJSON;
  bottom: TableCellMarginJSON;
  right: TableCellMarginJSON;
};

export type CellMarginsJSON = {
  top: TableCellMarginJSON;
  left: TableCellMarginJSON;
  bottom: TableCellMarginJSON;
  right: TableCellMarginJSON;
};

export type TablePropertyJSON = {
  width: {
    width: number;
    widthType: WidthType;
  } | null;
  justification: "left" | "center" | "right";
  borders?: {
    top?: BorderJSON | null;
    left?: BorderJSON | null;
    bottom?: BorderJSON | null;
    right?: BorderJSON | null;
    insideH?: BorderJSON | null;
    insideV?: BorderJSON | null;
  } | null;
  margins?: TableCellMarginsJSON | null;
  indent?: {
    width: number;
    widthType: WidthType;
  } | null;
  style?: string | null;
  layout?: TableLayoutType | null;
  position?: TablePositionPropertyJSON;
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
