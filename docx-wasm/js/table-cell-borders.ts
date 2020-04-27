import { BorderPosition, TableCellBorder } from "./table-cell-border";

export class TableCellBorders {
  top: TableCellBorder | null = new TableCellBorder("Top");
  left: TableCellBorder | null = new TableCellBorder("Left");
  bottom: TableCellBorder | null = new TableCellBorder("Bottom");
  right: TableCellBorder | null = new TableCellBorder("Right");
  insideH: TableCellBorder | null = new TableCellBorder("InsideH");
  insideV: TableCellBorder | null = new TableCellBorder("InsideV");

  set(border: TableCellBorder) {
    switch (border.position) {
      case "Top":
        this.top = border;
      case "Left":
        this.left = border;
      case "Bottom":
        this.bottom = border;
      case "Right":
        this.right = border;
      case "InsideH":
        this.insideH = border;
      case "InsideV":
        this.insideV = border;
    }
    return this;
  }

  clear(position: BorderPosition) {
    let nil = new TableCellBorder(position).border_type("Nil");
    switch (position) {
      case "Top":
        this.top = nil;
      case "Left":
        this.left = nil;
      case "Bottom":
        this.bottom = nil;
      case "Right":
        this.right = nil;
      case "InsideH":
        this.insideH = nil;
      case "InsideV":
        this.insideV = nil;
    }
    return this;
  }

  clearAll() {
    this.top = new TableCellBorder("Top").border_type("Nil");
    this.left = new TableCellBorder("Left").border_type("Nil");
    this.bottom = new TableCellBorder("Bottom").border_type("Nil");
    this.right = new TableCellBorder("Right").border_type("Nil");
    this.insideH = new TableCellBorder("InsideH").border_type("Nil");
    this.insideV = new TableCellBorder("InsideV").border_type("Nil");
    return this;
  }
}
