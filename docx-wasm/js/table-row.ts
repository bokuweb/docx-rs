import { TableCell } from "./table-cell";

export type HeightRule = "auto" | "atLeast" | "exact";

export class TableRow {
  cells: TableCell[] = [];
  hasNumberings = false;
  height: number | null = null;
  hRule: HeightRule = "atLeast";

  addCell(cell: TableCell) {
    if (cell.hasNumberings) {
      this.hasNumberings = true;
    }
    this.cells.push(cell);
    return this;
  }

  rowHeight(h: number) {
    this.height = h;
    return this;
  }

  heightRule(r: HeightRule) {
    this.hRule = r;
    return this;
  }
}
