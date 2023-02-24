import { Paragraph } from "./paragraph";
import { Table } from "./table";
import { Shading } from "./shading";
import { TableCellBorders, PositionKeys } from "./table-cell-borders";
import { TableCellBorderPosition, TableCellBorder } from "./table-cell-border";
import * as wasm from "./pkg";
import { convertBorderType } from "./run";
import { build } from "./builder";

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
  shading?: Shading;
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

  shading(type: string, color: string, fill: string) {
    const s = new Shading();
    s.color(color);
    s.fill(fill);
    s.type(type);
    this.property.shading = s;
    return this;
  }

  textDirection(t: TextDirectionType) {
    this.property.textDirection = t;
    return this;
  }

  setBorder(border: TableCellBorder) {
    this.property.borders[border.position.toLowerCase() as PositionKeys] =
      border;
    return this;
  }

  clearBorder(position: TableCellBorderPosition) {
    this.property.borders[position.toLowerCase() as PositionKeys] =
      new TableCellBorder(position).border_type("nil");
    return this;
  }

  buildCellBorders(cell: wasm.TableCell): wasm.TableCell {
    if (this.property.borders.top) {
      const border = wasm
        .createTableCellBorder(wasm.TableCellBorderPosition.Top)
        .size(this.property.borders.top._size)
        .color(this.property.borders.top._color)
        .border_type(convertBorderType(this.property.borders.top._border_type));
      cell = cell.set_border(border);
    }

    if (this.property.borders.right) {
      const border = wasm
        .createTableCellBorder(wasm.TableCellBorderPosition.Right)
        .size(this.property.borders.right._size)
        .color(this.property.borders.right._color)
        .border_type(
          convertBorderType(this.property.borders.right._border_type)
        );
      cell = cell.set_border(border);
    }

    if (this.property.borders.bottom) {
      const border = wasm
        .createTableCellBorder(wasm.TableCellBorderPosition.Bottom)
        .size(this.property.borders.bottom._size)
        .color(this.property.borders.bottom._color)
        .border_type(
          convertBorderType(this.property.borders.bottom._border_type)
        );
      cell = cell.set_border(border);
    }

    if (this.property.borders.left) {
      const border = wasm
        .createTableCellBorder(wasm.TableCellBorderPosition.Left)
        .size(this.property.borders.left._size)
        .color(this.property.borders.left._color)
        .border_type(
          convertBorderType(this.property.borders.left._border_type)
        );
      cell = cell.set_border(border);
    }

    if (this.property.borders.insideH) {
      const border = wasm
        .createTableCellBorder(wasm.TableCellBorderPosition.InsideH)
        .size(this.property.borders.insideH._size)
        .color(this.property.borders.insideH._color)
        .border_type(
          convertBorderType(this.property.borders.insideH._border_type)
        );
      cell = cell.set_border(border);
    }

    if (this.property.borders.insideV) {
      const border = wasm
        .createTableCellBorder(wasm.TableCellBorderPosition.InsideV)
        .size(this.property.borders.insideV._size)
        .color(this.property.borders.insideV._color)
        .border_type(
          convertBorderType(this.property.borders.insideV._border_type)
        );
      cell = cell.set_border(border);
    }

    if (this.property.borders.tl2br) {
      const border = wasm
        .createTableCellBorder(wasm.TableCellBorderPosition.Tl2br)
        .size(this.property.borders.tl2br._size)
        .color(this.property.borders.tl2br._color)
        .border_type(
          convertBorderType(this.property.borders.tl2br._border_type)
        );
      cell = cell.set_border(border);
    }

    if (this.property.borders.tr2bl) {
      const border = wasm
        .createTableCellBorder(wasm.TableCellBorderPosition.Tr2bl)
        .size(this.property.borders.tr2bl._size)
        .color(this.property.borders.tr2bl._color)
        .border_type(
          convertBorderType(this.property.borders.tr2bl._border_type)
        );
      cell = cell.set_border(border);
    }

    return cell;
  }

  build() {
    let cell = wasm.createTableCell();
    this.children.forEach((c) => {
      if (c instanceof Paragraph) {
        cell = cell.add_paragraph(build(c));
      } else if (c instanceof Table) {
        const table = c.build();
        cell = cell.add_table(table);
      }
    });

    if (this.property.verticalMerge === "continue") {
      cell = cell.vertical_merge(wasm.VMergeType.Continue);
    } else if (this.property.verticalMerge === "restart") {
      cell = cell.vertical_merge(wasm.VMergeType.Restart);
    }

    switch (this.property.verticalAlign) {
      case "top": {
        cell = cell.vertical_align(wasm.VAlignType.Top);
        break;
      }
      case "center": {
        cell = cell.vertical_align(wasm.VAlignType.Center);
        break;
      }
      case "bottom": {
        cell = cell.vertical_align(wasm.VAlignType.Bottom);
        break;
      }
    }

    if (typeof this.property.gridSpan !== "undefined") {
      cell = cell.grid_span(this.property.gridSpan);
    }

    if (typeof this.property.width !== "undefined") {
      cell = cell.width(this.property.width);
    }

    if (typeof this.property.textDirection !== "undefined") {
      cell = cell.text_direction(
        toTextDirectionWasmType(this.property.textDirection)
      );
    }

    if (typeof this.property.borders !== "undefined") {
      cell = this.buildCellBorders(cell);
    }

    if (typeof this.property.shading !== "undefined") {
      cell = cell.shading(
        this.property.shading._type,
        this.property.shading._color,
        this.property.shading._fill
      );
    }

    return cell;
  }
}
