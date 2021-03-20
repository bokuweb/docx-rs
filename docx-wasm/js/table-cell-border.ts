import { BorderType } from "./border";

export type TableCellBorderPosition =
  | "left"
  | "right"
  | "top"
  | "bottom"
  | "insideH"
  | "insideV"
  | "tl2br"
  | "tr2bl";

export class TableCellBorder {
  _border_type: BorderType;
  _size = 2;
  _color = "000000";
  position: TableCellBorderPosition;
  space = 0;

  constructor(position: TableCellBorderPosition) {
    this.position = position;
  }

  color(color: string) {
    this._color = color;
    return this;
  }

  size(size: number) {
    this._size = size;
    return this;
  }

  border_type(border_type: BorderType) {
    this._border_type = border_type;
    return this;
  }
}
