import { TableCell } from "./table-cell";

export class TableRow {
  cells: TableCell[] = [];
  hasNumberings = false;
  height: number | null = null;

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
}
