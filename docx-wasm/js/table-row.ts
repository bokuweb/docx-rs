import { TableCell } from "./table-cell";

export class TableRow {
  cells: TableCell[] = [];
  hasNumberings = false;

  addCell(cell: TableCell) {
    if (cell.hasNumberings) {
      this.hasNumberings = true;
    }
    this.cells.push(cell);
    this;
  }
}
