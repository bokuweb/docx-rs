import { TableRow } from "./table-row";

export type TableAlignmentType = "center" | "left" | "right";
export type TableLayoutType = "fixed" | "autofit";

export type TableProperty = {
  indent?: number;
  align?: TableAlignmentType;
  width?: number;
  cellMargins?: {
    top: number;
    left: number;
    bottom: number;
    right: number;
  };
  layout?: TableLayoutType;
};

export class Table {
  hasNumberings = false;
  rows: TableRow[] = [];
  grid: number[] = [];
  property: TableProperty = {};

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
    this.property.cellMargins = { top, left, bottom, right };
    return this;
  }
}
