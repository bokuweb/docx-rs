export type BorderType =
  | "nil"
  | "none"
  | "single"
  | "thick"
  | "double"
  | "dotted"
  | "dashed"
  | "dotDash"
  | "dotDotDash"
  | "triple"
  | "thinThickSmallGap"
  | "thickThinSmallGap"
  | "thinThickThinSmallGap"
  | "thinThickMediumGap"
  | "thickThinMediumGap"
  | "thinThickThinMediumGap"
  | "thinThickLargeGap"
  | "thickThinLargeGap"
  | "thinThickThinLargeGap"
  | "wave"
  | "doubleWave"
  | "dashSmallGap"
  | "dashDotStroked"
  | "threeDEmboss"
  | "threeDEngrave"
  | "outset"
  | "inset"
  | "apples"
  | "archedScallops"
  | "babyPacifier"
  | "babyRattle";

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
