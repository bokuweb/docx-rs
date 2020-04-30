export type BorderType =
  | "Nil"
  | "None"
  | "Single"
  | "Thick"
  | "Double"
  | "Dotted"
  | "Dashed"
  | "DotDash"
  | "DotDotDash"
  | "Triple";

export type BorderPosition =
  | "left"
  | "right"
  | "top"
  | "bottom"
  | "insideH"
  | "insideV";

export class TableCellBorder {
  _border_type: BorderType;
  _size = 2;
  _color = "000000";
  position: BorderPosition;
  space = 0;

  constructor(position: BorderPosition) {
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
