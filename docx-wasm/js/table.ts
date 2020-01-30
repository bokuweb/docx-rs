import { TableRow } from "./table-row";

export type TableAlignmentType = "center" | "left" | "right";

export type TableProperty = {
  indent?: number;
  align?: TableAlignmentType;
  width?: number;
};

export class Table {
  rows: TableRow[] = [];
  grid: number[] = [];
  property: TableProperty;

  addRow(row: TableRow) {
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

  width(w: number) {
    this.property.width = w;
    return this;
  }
}
