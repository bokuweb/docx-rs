import { TableCell } from "./table-cell";

export class TableRow {
  cells: TableCell[] = [];

  addCell(cell: TableCell) {
    this.cells.push(cell);
    this;
  }
}
