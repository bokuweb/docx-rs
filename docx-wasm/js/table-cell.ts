import { Paragraph } from "./paragraph";
import { Table } from "./table";
import { TableCellBorders, PositionKeys } from "./table-cell-borders";
import { BorderPosition, TableCellBorder } from "./table-cell-border";
import * as wasm from "./pkg";

export type VMergeType = "restart" | "continue";

export type VAlignType = "top" | "center" | "bottom";

export type TextDirectionType =
  | "lr"
  | "lrV"
  | "rl"
  | "rlV"
  | "tb"
  | "tbV"
  | "tbRlV"
  | "tbRl"
  | "btLr"
  | "lrTbV";

export const toTextDirectionWasmType = (
  t: TextDirectionType
): wasm.TextDirectionType => {
  switch (t) {
    case "lr":
      return wasm.TextDirectionType.Lr;
    case "lrV":
      return wasm.TextDirectionType.LrV;
    case "rl":
      return wasm.TextDirectionType.Rl;
    case "rlV":
      return wasm.TextDirectionType.RlV;
    case "tb":
      return wasm.TextDirectionType.Tb;
    case "tbV":
      return wasm.TextDirectionType.TbV;
    case "tbRlV":
      return wasm.TextDirectionType.TbRlV;
    case "tbRl":
      return wasm.TextDirectionType.TbRl;
    case "btLr":
      return wasm.TextDirectionType.BtLr;
    case "lrTbV":
      return wasm.TextDirectionType.LrTbV;
    default:
      throw new Error("unreachable");
  }
};

export type CellProperty = {
  borders: TableCellBorders;
  verticalMerge?: VMergeType;
  verticalAlign?: VAlignType;
  gridSpan?: number;
  width?: number;
  textDirection?: TextDirectionType;
};

export class TableCell {
  children: (Paragraph | Table)[] = [];
  hasNumberings = false;
  property: CellProperty = {
    borders: new TableCellBorders(),
  };

  addParagraph(p: Paragraph) {
    if (p.hasNumberings) {
      this.hasNumberings = true;
    }
    this.children.push(p);
    return this;
  }

  addTable(t: Table) {
    if (t.hasNumberings) {
      this.hasNumberings = true;
    }
    this.children.push(t);
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

  textDirection(t: TextDirectionType) {
    this.property.textDirection = t;
    return this;
  }

  setBorder(position: BorderPosition, border: TableCellBorder) {
    this.property.borders[position.toLowerCase() as PositionKeys] = border;
    return this;
  }

  clearBorder(position: BorderPosition) {
    this.property.borders[
      position.toLowerCase() as PositionKeys
    ] = new TableCellBorder(position).border_type("nil");
    return this;
  }
}
