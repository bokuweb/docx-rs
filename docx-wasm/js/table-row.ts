import { TableCell } from "./table-cell";

export type HeightRule = "auto" | "atLeast" | "exact";

export class TableRow {
  cells: TableCell[] = [];
  hasNumberings = false;
  height: number | null = null;
  hRule: HeightRule | null = null;
  del: { author: string; date: string } | null = null;
  ins: { author: string; date: string } | null = null;

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

  delete(author: string, date: string) {
    this.del = { author, date };
    return this;
  }

  insert(author: string, date: string) {
    this.ins = { author, date };
    return this;
  }
}
