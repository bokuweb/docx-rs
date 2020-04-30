import { Paragraph } from "./paragraph";
import { TableCellBorders, PositionKeys } from "./table-cell-borders";
import { BorderPosition, TableCellBorder } from "./table-cell-border";

export type VMergeType = "restart" | "continue";

export type VAlignType = "top" | "center" | "bottom";

export type CellProperty = {
  borders: TableCellBorders;
  verticalMerge?: VMergeType;
  verticalAlign?: VAlignType;
  gridSpan?: number;
  width?: number;
};

export class TableCell {
  children: Paragraph[] = [];
  property: CellProperty = {
    borders: new TableCellBorders(),
  };

  addParagraph(p: Paragraph) {
    this.children.push(p);
    return this;
  }

  verticalMerge(t: VMergeType) {
    this.property.verticalMerge = t;
    return this;
  }

  verticalAlign(t: VAlignType) {
    this.property.verticalAlign = t;
    return this;
  }

  gridSpan(v: number) {
    this.property.gridSpan = v;
    return this;
  }

  width(v: number) {
    this.property.width = v;
    return this;
  }

  setBorder(position: BorderPosition, border: TableCellBorder) {
    this.property.borders[position.toLowerCase() as PositionKeys] = border;
    return this;
  }

  clearBorder(position: BorderPosition) {
    this.property.borders[
      position.toLowerCase() as PositionKeys
    ] = new TableCellBorder(position).border_type("Nil");
    return this;
  }
}
