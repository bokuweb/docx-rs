import { WidthType } from ".";
import { TableRow } from "./table-row";

export type TableAlignmentType = "center" | "left" | "right";
export type TableLayoutType = "fixed" | "autofit";

export type TableProperty = {
  indent?: number;
  align?: TableAlignmentType;
  width?: number;
  cellMargins: {
    top: { val: number; type: WidthType };
    left: { val: number; type: WidthType };
    bottom: { val: number; type: WidthType };
    right: { val: number; type: WidthType };
  };
  layout?: TableLayoutType;
};

export const createDefaultTableCellMargins = () => {
  return {
    top: { val: 0, type: "dxa" },
    left: { val: 55, type: "dxa" },
    bottom: { val: 0, type: "dxa" },
    right: { val: 55, type: "dxa" },
  } as const;
};

export class Table {
  hasNumberings = false;
  rows: TableRow[] = [];
  grid: number[] = [];
  property: TableProperty = {
    cellMargins: createDefaultTableCellMargins(),
  };

  addRow(row: TableRow) {
    if (row.hasNumberings) {
      this.hasNumberings = true;
    }
    this.rows.push(row);
    return this;
  }

  setGrid(grid: number[]) {
    this.grid = grid;
    return this;
  }

  indent(v: number) {
    this.property.indent = v;
    return this;
  }

  align(v: TableAlignmentType) {
    this.property.align = v;
    return this;
  }

  layout(l: TableLayoutType) {
    this.property.layout = l;
    return this;
  }

  width(w: number) {
    this.property.width = w;
    return this;
  }

  cellMargins(top: number, right: number, bottom: number, left: number) {
    this.property.cellMargins = {
      top: { val: top, type: "dxa" },
      left: { val: left, type: "dxa" },
      bottom: { val: bottom, type: "dxa" },
      right: { val: right, type: "dxa" },
    };
    return this;
  }

  cellMarginTop(v: number, t: WidthType) {
    this.property.cellMargins.top = { val: v, type: t };
    return this;
  }

  cellMarginLeft(v: number, t: WidthType) {
    this.property.cellMargins.left = { val: v, type: t };
    return this;
  }

  cellMarginRight(v: number, t: WidthType) {
    this.property.cellMargins.right = { val: v, type: t };
    return this;
  }

  cellMarginBottom(v: number, t: WidthType) {
    this.property.cellMargins.bottom = { val: v, type: t };
    return this;
  }
}
