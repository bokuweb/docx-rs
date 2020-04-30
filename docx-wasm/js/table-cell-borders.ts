import { BorderPosition, TableCellBorder } from "./table-cell-border";

export type PositionKeys =
  | "top"
  | "left"
  | "bottom"
  | "right"
  | "insideH"
  | "insideV";

export class TableCellBorders {
  top: TableCellBorder | null = new TableCellBorder("top");
  left: TableCellBorder | null = new TableCellBorder("left");
  bottom: TableCellBorder | null = new TableCellBorder("bottom");
  right: TableCellBorder | null = new TableCellBorder("right");
  insideH: TableCellBorder | null = new TableCellBorder("insideH");
  insideV: TableCellBorder | null = new TableCellBorder("insideV");

  set(border: TableCellBorder) {
    switch (border.position) {
      case "top":
        this.top = border;
      case "left":
        this.left = border;
      case "bottom":
        this.bottom = border;
      case "right":
        this.right = border;
      case "insideH":
        this.insideH = border;
      case "insideV":
        this.insideV = border;
    }
    return this;
  }

  clear(position: BorderPosition) {
    let nil = new TableCellBorder(position).border_type("Nil");
    switch (position) {
      case "top":
        this.top = nil;
      case "left":
        this.left = nil;
      case "bottom":
        this.bottom = nil;
      case "right":
        this.right = nil;
      case "insideH":
        this.insideH = nil;
      case "insideV":
        this.insideV = nil;
    }
    return this;
  }

  clearAll() {
    this.top = new TableCellBorder("top").border_type("Nil");
    this.left = new TableCellBorder("left").border_type("Nil");
    this.bottom = new TableCellBorder("bottom").border_type("Nil");
    this.right = new TableCellBorder("right").border_type("Nil");
    this.insideH = new TableCellBorder("insideH").border_type("Nil");
    this.insideV = new TableCellBorder("insideV").border_type("Nil");
    return this;
  }
}
